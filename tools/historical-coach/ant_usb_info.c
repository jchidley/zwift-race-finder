/*
 * Bounded ANT USB version/capability probe. No network keys, channel
 * assignment, search, open-channel or data-transmission commands.
 * Build: cc -std=c11 -Wall -Wextra -Werror ant_usb_info.c -lusb-1.0 -o ant-usb-info
 * Run with USB access: ant-usb-info 1008 | 1009
 */
#define _POSIX_C_SOURCE 200809L
#include <libusb-1.0/libusb.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

static double seconds(void) {
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return (double)t.tv_sec + (double)t.tv_nsec / 1e9;
}

static int send_command(libusb_device_handle *h, unsigned char ep,
                        unsigned char id, const unsigned char *data, int len) {
    /* Only these diagnostic commands are permitted. */
    if (!((id == 0x4a && len == 1 && data[0] == 0) ||
          (id == 0x4d && len == 2 && data[0] == 0 &&
           (data[1] == 0x3e || data[1] == 0x54)))) {
        fprintf(stderr, "Refused non-diagnostic command\n");
        return -1;
    }
    unsigned char frame[6] = {0xa4, (unsigned char)len, id, 0, 0, 0};
    memcpy(frame + 3, data, (size_t)len);
    unsigned char checksum = 0;
    for (int i = 0; i < len + 3; ++i) checksum ^= frame[i];
    frame[len + 3] = checksum;
    int actual = 0;
    int r = libusb_bulk_transfer(h, ep, frame, len + 4, &actual, 1000);
    if (r || actual != len + 4) {
        fprintf(stderr, "USB write: %s (%d bytes)\n", libusb_error_name(r), actual);
        return -1;
    }
    return 0;
}

static int receive(libusb_device_handle *h, unsigned char ep, unsigned char wanted) {
    unsigned char pending[2048];
    size_t used = 0;
    double deadline = seconds() + 3.0;
    while (seconds() < deadline) {
        unsigned char chunk[512];
        int actual = 0;
        int r = libusb_bulk_transfer(h, ep, chunk, sizeof chunk, &actual, 500);
        if (r && r != LIBUSB_ERROR_TIMEOUT) {
            fprintf(stderr, "USB read: %s\n", libusb_error_name(r));
            return -1;
        }
        if ((size_t)actual > sizeof pending - used) {
            fprintf(stderr, "Receive buffer overflow\n");
            return -1;
        }
        memcpy(pending + used, chunk, (size_t)actual);
        used += (size_t)actual;
        while (used) {
            if (pending[0] != 0xa4) {
                memmove(pending, pending + 1, --used);
                continue;
            }
            if (used < 4) break;
            size_t frame_len = (size_t)pending[1] + 4;
            if (used < frame_len) break;
            unsigned char checksum = 0;
            for (size_t i = 0; i < frame_len; ++i) checksum ^= pending[i];
            if (checksum) {
                fprintf(stderr, "Invalid ANT frame checksum\n");
                return -1;
            }
            unsigned char id = pending[2];
            printf("ANT response %02x payload:", id);
            for (size_t i = 3; i < frame_len - 1; ++i) printf(" %02x", pending[i]);
            putchar('\n');
            if (id == 0x3e) {
                printf("Version: ");
                for (size_t i = 3; i < frame_len - 1 && pending[i]; ++i)
                    putchar(pending[i] >= 32 && pending[i] <= 126 ? pending[i] : '?');
                putchar('\n');
            }
            if (id == 0x54 && pending[1] >= 4) {
                printf("Max channels: %u; max networks: %u; standard options: 0x%02x\n",
                       pending[3], pending[4], pending[5]);
            }
            if (id == wanted) return 0;
            memmove(pending, pending + frame_len, used - frame_len);
            used -= frame_len;
        }
    }
    fprintf(stderr, "Timed out waiting for ANT response %02x\n", wanted);
    return -1;
}

int main(int argc, char **argv) {
    if (argc != 2 || (strcmp(argv[1], "1008") && strcmp(argv[1], "1009"))) {
        fprintf(stderr, "Usage: %s 1008|1009\n", argv[0]);
        return 2;
    }
    unsigned short pid = (unsigned short)strtoul(argv[1], NULL, 16);
    libusb_context *ctx = NULL;
    libusb_device_handle *h = NULL;
    struct libusb_config_descriptor *cfg = NULL;
    int result = 1, detached = 0, claimed = 0;
    unsigned char in = 0, out = 0;
    int r = libusb_init(&ctx);
    if (r) { fprintf(stderr, "libusb init: %s\n", libusb_error_name(r)); return 1; }
    /* Refuse ambiguity rather than selecting an arbitrary identical stick. */
    libusb_device **devices = NULL;
    ssize_t count = libusb_get_device_list(ctx, &devices);
    if (count < 0) goto cleanup;
    int matches = 0;
    libusb_device *selected = NULL;
    for (ssize_t i = 0; i < count; ++i) {
        struct libusb_device_descriptor desc;
        if (!libusb_get_device_descriptor(devices[i], &desc) &&
            desc.idVendor == 0x0fcf && desc.idProduct == pid) {
            ++matches;
            selected = devices[i];
        }
    }
    if (matches != 1) {
        fprintf(stderr, "Expected exactly one 0fcf:%04x stick; found %d\n", pid, matches);
        libusb_free_device_list(devices, 1);
        goto cleanup;
    }
    r = libusb_open(selected, &h);
    libusb_free_device_list(devices, 1);
    if (r) { fprintf(stderr, "USB open: %s\n", libusb_error_name(r)); goto cleanup; }
    r = libusb_get_active_config_descriptor(libusb_get_device(h), &cfg);
    if (r) goto cleanup;
    for (int i = 0; i < cfg->bNumInterfaces; ++i) {
        const struct libusb_interface *iface = &cfg->interface[i];
        for (int a = 0; a < iface->num_altsetting; ++a) {
            const struct libusb_interface_descriptor *alt = &iface->altsetting[a];
            if (alt->bInterfaceNumber != 0 || alt->bAlternateSetting != 0) continue;
            for (int e = 0; e < alt->bNumEndpoints; ++e) {
                const struct libusb_endpoint_descriptor *ep = &alt->endpoint[e];
                if ((ep->bmAttributes & 3) != LIBUSB_TRANSFER_TYPE_BULK) continue;
                if (ep->bEndpointAddress & LIBUSB_ENDPOINT_IN) in = ep->bEndpointAddress;
                else out = ep->bEndpointAddress;
            }
        }
    }
    if (!in || !out) { fprintf(stderr, "Missing bulk endpoint pair\n"); goto cleanup; }
    r = libusb_kernel_driver_active(h, 0);
    if (r < 0) goto cleanup;
    if (r == 1) {
        r = libusb_detach_kernel_driver(h, 0);
        if (r) { fprintf(stderr, "Detach: %s\n", libusb_error_name(r)); goto cleanup; }
        detached = 1;
    }
    r = libusb_claim_interface(h, 0);
    if (r) { fprintf(stderr, "Claim: %s\n", libusb_error_name(r)); goto cleanup; }
    claimed = 1;
    printf("Opened 0fcf:%04x, bulk IN=%02x OUT=%02x\n", pid, in, out);
    unsigned char reset[] = {0};
    unsigned char version[] = {0, 0x3e};
    unsigned char capabilities[] = {0, 0x54};
    if (send_command(h, out, 0x4a, reset, 1) || receive(h, in, 0x6f)) goto cleanup;
    if (send_command(h, out, 0x4d, version, 2) || receive(h, in, 0x3e)) goto cleanup;
    if (send_command(h, out, 0x4d, capabilities, 2) || receive(h, in, 0x54)) goto cleanup;
    result = 0;
cleanup:
    if (claimed && (r = libusb_release_interface(h, 0))) {
        fprintf(stderr, "Release: %s\n", libusb_error_name(r)); result = 1;
    }
    if (detached) {
        r = libusb_attach_kernel_driver(h, 0);
        if (r || libusb_kernel_driver_active(h, 0) != 1) {
            fprintf(stderr, "Driver restoration failed: %s\n", libusb_error_name(r)); result = 1;
        } else puts("Original kernel driver reattached and verified.");
    }
    if (cfg) libusb_free_config_descriptor(cfg);
    if (h) libusb_close(h);
    libusb_exit(ctx);
    return result;
}
