# Sauce headless/offline and actual download review — 2026-10-30

This is the 2026-10-30 archive/source inspection only. [Later live capture and FIT-alignment evidence](../../STATUS.md#recovered-sauce-evidence--2026-11-04) was recovered on 2026-11-04; the no-launch statements below describe this inspection, not all subsequent testing.

## Result

**Headless is implemented. Offline-server routing exists in the code, but the
official stable download is not configured for it by default.** We have not
demonstrated a no-internet/zwift-offline deployment. Do not label that impossible,
but do not promise supported offline operation from these findings.

Owner requires no additional payments beyond Zwift. Official Sauce product page
explicitly requires monthly Patreon membership for the packaged product.
Source-build rights/behavior must be evaluated separately: current source and
stable download have different license metadata.

## What was inspected

- Local checkout `~/git/sauce4zwift`, commit
  `bb53edfc5050150cd669e56fc60f6d5726a534cd`, package 2.4.0-alpha.6.
- Actual official product page:
  <https://www.sauce.llc/products/sauce4zwift/>
- Actual download page:
  <https://www.sauce.llc/products/sauce4zwift/download>
- Download page JavaScript fetches:
  <https://api.github.com/repos/SauceLLC/sauce4zwift-releases/releases/latest>
- At inspection that endpoint selected stable **v2.3.4**; prerelease listing also
  contained 2.4.0-alpha.6. A matching source tag v2.3.4 returned 404, so a tag
  comparison could not substitute for inspecting the package.
- Downloaded official macOS ZIP **only as an inspectable archive**, not a target
  runtime, since unzip was available but no AppImage inspection utility was.
  No binary/postinstall/launcher was executed.
- Artifact:
  <https://github.com/SauceLLC/sauce4zwift-releases/releases/download/v2.3.4/sauce4zwift-2.3.4.zip>
- Size reported by release API: **216,296,050 bytes**.
- Downloaded ZIP SHA-256:
  `a86d0bea7c06a24af188cb09ea23a2e4fe609410c9444ac877106e4ad39a196b`.
  This identifies the inspected artifact; it is not independent signature validation.
- Temporary inspection directory:
  `/tmp/sauce-download-review-irp5rZ/`.
  Extracted only `Contents/Resources/app.asar`, then read selected embedded text
  files with a Node ASAR-header reader. Scanned src/pages text for offline-server
  mentions. Native/unpacked code and the whole dependency tree were not audited.
  Temporary files are not durable project evidence or copied into Git.

## Headless: source-confirmed, without overlay

Local `src/loader.js:235–266` routes `--headless` to `src/headless.mjs`
under Electron's Node mode. Headless supplies empty window/profile/widget RPC
responses and uses the shared backend, monitor/statistics and web server.

`src/headless.mjs`:
- initializes separate state under `~/.sauce4zwift`;
- requires main and monitor username/password, allowing environment variables;
- authenticates both accounts;
- starts the shared application.
The stable downloaded package also includes this headless module and two-account
authentication calls (`src/headless.mjs:132–133` in its embedded text).

Useful API paths in the local source:
- `/api`: directory;
- `/api/athlete/v2/self`: rider data/resources;
- `/api/athlete/streams/v1/self`: streams;
- `/api/nearby/v1`: nearby athletes;
- `/api/ws/events`: event subscriptions.

`--disable-game-connection` means disable the Companion protocol service, not
disable the encrypted watching client. Sauce being headless says nothing about
the Zwift game client running headlessly or accelerated.

Local `src/webserver.mjs:584` binds to **0.0.0.0**. Inspected HTTP setup and WS
upgrade path did not show an authentication gate. Review/restrict exposure before
launch; local UI integration may need a same-origin proxy. Keep credentials out
of CLI arguments/logs.

## Offline-server evidence in the download

Embedded `src/zwift.mjs` in stable v2.3.4:

- Line 16: `const zOffline = null;  // 'localhost';`
- Authentication host (line 340) and token refresh (line 382):
  `this.host || zOffline || 'secure.zwift.com'`.
- General request host (line 467):
  `options.host || this.host || zOffline || 'us-or-rly101.zwift.com'`.
- An additional comment mentions inspecting behavior with zwift-offline.

Local alpha source has the same switch at line 17 and an API-level
`authenticate(..., {host, scheme})` override.
**But the inspected headless CLI has no offline/server-host option**, and its
calls do not supply a host override. Stable normal startup likewise passes no
host override for either account.

This is real evidence of a contemplated/developer offline-server route, not
merely a mistaken product name. It is still disabled and not a complete supported
configuration in the inspected launch path. We did not edit the switch, redirect
DNS/hosts, change certificates or connect to an offline server.

Follow-up would need to establish:
1. Which Sauce version/build and supported configuration the owner has in mind.
2. Authentication, TLS and relay endpoint compatibility with the selected
   zwift-offline server, including the separate monitoring account.
3. Other external accesses (e.g. local source fetches the Zwift game dictionary
   from its CDN), cache availability and update/membership behavior.
4. A bounded no-internet test with useful telemetry—not just successful process
   startup. No such test was authorized/performed here.

## Local versus offline versus free

The official product page says:
- Patreon monthly patronage is required.
- The second Zwift account does not need paid billing.
- Zwift data travels directly between the local computer and Zwift servers,
  without Sauce cloud storage/proxying.

Thus **client-side/local** is not the same as **no-internet**, and **free second
Zwift account** is not the same as **free Sauce product**.
The page's trial billing advice was not independently verified and is not relied on.

The downloaded stable package's `package.json` says **UNLICENSED**.
The local newer source package says **GPL-3.0-only** and contains GPL text.
Do not silently apply the newer source declaration to the older binary or
redistribute package contents based on this review. Source-build availability
does not establish identical entitlement or offline behavior.

## Fit with our app

Sauce, if separately selected and compatible, is a telemetry backend, not the
bike resistance controller or required overlay. Existing native host remains
the sole Pico/USB/control owner:

```text
Zwift → Sauce headless/API → coaching / existing dashboard
Existing native host → Pico → physical resistance
```

No software install, account login, app launch, physical control or external
write was performed. Archive inspection is not runtime verification.

Related: [bridge comparison](BRIDGE_COMPARISON_2026-10-30.md) and
[companion catalogue](COMPANION_TOOLS_2026-10-30.md).
