# Gymnasticon Bot Mode Setup Guide

## Overview

Gymnasticon can run in "bot" mode where it broadcasts as a Bluetooth power meter and cadence sensor without needing a real bike. This is useful for testing or creating a virtual bike that can be controlled programmatically.

## Bot Mode Features

- Broadcasts as a Bluetooth LE Cycling Power Service
- Broadcasts as a Bluetooth LE Cycling Speed and Cadence Service  
- Optional ANT+ broadcasting (requires ANT+ USB stick)
- Power and cadence can be controlled via UDP messages
- Appears to Zwift and other apps as a standard power meter

**Connection Notes**: 
- **Reliability varies**: Sometimes Bluetooth works better, sometimes ANT+ does
- **Connection order may matter**: Sometimes the order of connecting Power/Cadence affects success
- **Always connect both**: Regardless of protocol, connect both Power AND Cadence sensors
- **If one fails, try the other**: If Bluetooth isn't working, try ANT+ and vice versa

## Basic Setup

### 1. Install Gymnasticon

```bash
# Install dependencies (on Debian/Ubuntu/Raspbian)
sudo apt-get install libudev-dev

# Install gymnasticon globally
npm install -g gymnasticon

# Give Node.js Bluetooth capabilities (avoids needing sudo)
sudo setcap cap_net_raw+eip $(eval readlink -f $(which node))
```

### 2. Run in Bot Mode

Basic bot mode with default values (0 watts, 0 rpm):
```bash
gymnasticon --bike bot
```

With initial power and cadence:
```bash
gymnasticon --bike bot --bot-power 200 --bot-cadence 90
```

## Configuration Options

### Command Line Options

- `--bike bot` - Enable bot mode
- `--bot-power <watts>` - Initial power output (default: 0)
- `--bot-cadence <rpm>` - Initial cadence (default: 0)
- `--bot-host <host>` - UDP listener host (default: '0.0.0.0')
- `--bot-port <port>` - UDP listener port (default: 3000)
- `--server-name <name>` - Bluetooth device name (default: 'Gymnasticon')
- `--server-adapter <adapter>` - Bluetooth adapter (default: 'hci0')
- `--power-scale <value>` - Scale power by multiplier (default: 1.0)
- `--power-offset <value>` - Add offset to power (default: 0)

### Configuration File

Create a `gymnasticon.json` file:
```json
{
  "bike": "bot",
  "bot-power": 200,
  "bot-cadence": 90,
  "bot-host": "0.0.0.0",
  "bot-port": 3000,
  "server-name": "MyVirtualBike"
}
```

**Minimal config for UDP-only control** (like `/etc/gymnasticon.json`):
```json
{
  "bike": "bot",
  "bot-power": 0,
  "bot-host": "0.0.0.0",
  "bot-port": 3000
}
```
Note: Without `bot-cadence`, it defaults to 0 or undefined. You'll need to set both via UDP.

Run with config file:
```bash
gymnasticon --config gymnasticon.json
```

## Controlling Power/Cadence via UDP

The bot listens on UDP port 3000 (or configured port) for JSON messages.

**Important**: Use the `-w1` flag and explicit IP `127.0.0.1` for reliable UDP delivery:

```bash
# Send power and cadence update (recommended syntax)
echo '{"power": 250, "cadence": 95}' | nc -u -w1 127.0.0.1 3000

# Update only power
echo '{"power": 300}' | nc -u -w1 127.0.0.1 3000

# Update only cadence  
echo '{"cadence": 85}' | nc -u -w1 127.0.0.1 3000
```

Python example:
```python
import socket
import json

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
data = {"power": 200, "cadence": 90}
sock.sendto(json.dumps(data).encode(), ("localhost", 3000))
```

## Running as a System Service

### 1. Create Service File

Save as `/etc/systemd/system/gymnasticon-bot.service`:
```ini
[Unit]
Description=Gymnasticon Bot Mode
After=bluetooth.target
Requires=bluetooth.target
StartLimitIntervalSec=0

[Service]
Type=simple
User=pi
Group=pi
ExecStart=/usr/bin/gymnasticon --bike bot --bot-power 0 --bot-cadence 0
RestartSec=1
Restart=always

# Required for Bluetooth access
AmbientCapabilities=CAP_NET_RAW CAP_NET_ADMIN
NoNewPrivileges=true

[Install]
WantedBy=multi-user.target
```

### 2. Enable and Start Service

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable auto-start on boot
sudo systemctl enable gymnasticon-bot

# Start the service
sudo systemctl start gymnasticon-bot

# Check status
sudo systemctl status gymnasticon-bot

# View logs
journalctl -u gymnasticon-bot -f
```

## Bluetooth Troubleshooting

### Check Bluetooth Status
```bash
# Check if Bluetooth is enabled
sudo systemctl status bluetooth

# List Bluetooth adapters
hciconfig -a

# Enable adapter if down
sudo hciconfig hci0 up

# Reset Bluetooth adapter
sudo hciconfig hci0 reset
```

### Common Issues

1. **"Operation not permitted" error**
   - Run with sudo OR use setcap command above
   - Ensure user is in 'bluetooth' group: `sudo usermod -a -G bluetooth $USER`

2. **"No compatible USB Bluetooth 4.0 device found"**
   - Check adapter with: `hciconfig -a`
   - Ensure adapter supports BLE 4.0+
   - Try different adapter name: `--server-adapter hci1`

3. **Device not appearing in Zwift**
   - Ensure no other device is using the adapter
   - Try resetting Bluetooth: `sudo systemctl restart bluetooth`
   - Check gymnasticon is advertising: `sudo hcitool lescan`
   - Try ANT+ instead if Bluetooth isn't working
   - Connection success can vary day to day

4. **Multiple Bluetooth adapters**
   - Use different adapters for server and bike connection:
     ```bash
     gymnasticon --bike bot --server-adapter hci0 --bike-adapter hci1
     ```

## UDP Control Troubleshooting

### Common UDP Issues

1. **UDP messages not working**
   - Use explicit IP and timeout: `echo '{"power":250}' | nc -u -w1 127.0.0.1 3000`
   - Avoid `localhost` - use `127.0.0.1` for reliability
   - The `-w1` flag is important for OpenBSD netcat

2. **"Socket already bound" error**
   - Stop any running gymnasticon instances: `sudo pkill -f gymnasticon`
   - Check what's using the port: `sudo lsof -i :3000`
   - Use a different port: `--bot-port 3001`

3. **Messages sent but no response**
   - Verify UDP server is listening: `sudo ss -ulnp | grep 3000`
   - Test with tcpdump: `sudo tcpdump -i lo -A udp port 3000`
   - Ensure both host and port are configured in bot mode

4. **Testing UDP connectivity**
   ```bash
   # Simple UDP echo test
   nc -u -l 3002  # Terminal 1: Listen
   echo "test" | nc -u -w1 127.0.0.1 3002  # Terminal 2: Send
   ```

## Testing the Setup

### 1. Verify Bluetooth Broadcasting
```bash
# Scan for BLE devices (should see "Gymnasticon")
sudo hcitool lescan

# Get more details about the device
sudo gatttool -b <MAC_ADDRESS> -I
connect
primary
```

### 2. Test with Zwift

**IMPORTANT**: Connection can be finicky - try these approaches:

**Method A - Bluetooth LE:**
1. Open Zwift
2. Go to pairing screen
3. Select "Gymnasticon" under **Power Source** (Bluetooth)
4. Select "Gymnasticon" under **Cadence** (same device)
5. Both must be connected
6. Click OK and start riding

**Method B - ANT+:**
1. Ensure ANT+ dongle is connected to your Zwift device
2. Go to pairing screen
3. Select the ANT+ device under **Power Source**
4. Select the same ANT+ device under **Cadence**
5. Both must be connected
6. Click OK and start riding

**Troubleshooting Connection Issues:**
- If Bluetooth fails, try ANT+
- If ANT+ fails, try Bluetooth
- Try different connection orders (Power first vs Cadence first)
- Restart gymnasticon between attempts
- Some days one protocol works better than the other

### 3. Send Test Data
```bash
# Terminal 1: Start gymnasticon
gymnasticon --bike bot

# Terminal 2: Send power/cadence updates
while true; do
  echo '{"power": 200, "cadence": 90}' | nc -u -w1 localhost 3000
  sleep 1
done
```

## Advanced Usage

### Programmatic Control Example

Create a script to simulate a workout:
```bash
#!/bin/bash
# workout.sh - Simulate a simple interval workout

# Warmup - 5 minutes at 150W
for i in {1..300}; do
  echo '{"power": 150, "cadence": 85}' | nc -u -w1 localhost 3000
  sleep 1
done

# Intervals - 8x (1min at 300W, 1min at 100W)
for interval in {1..8}; do
  # High intensity
  for i in {1..60}; do
    echo '{"power": 300, "cadence": 95}' | nc -u -w1 localhost 3000
    sleep 1
  done
  
  # Recovery
  for i in {1..60}; do
    echo '{"power": 100, "cadence": 70}' | nc -u -w1 localhost 3000
    sleep 1
  done
done

# Cool down - 5 minutes at 100W
for i in {1..300}; do
  echo '{"power": 100, "cadence": 80}' | nc -u -w1 localhost 3000
  sleep 1
done
```

## Integration with Zwift Race Finder

You could potentially use gymnasticon bot mode to:
1. Test race pacing strategies by controlling power output
2. Simulate different Racing Scores by adjusting power
3. Create automated bots for testing multiplayer features
4. Generate consistent power data for algorithm validation

Example integration:
```rust
// Send power updates to gymnasticon from Rust
use std::net::UdpSocket;

fn update_virtual_bike(power: u16, cadence: u16) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let msg = format!(r#"{{"power": {}, "cadence": {}}}"#, power, cadence);
    socket.send_to(msg.as_bytes(), "127.0.0.1:3000")?;
    Ok(())
}
```

## Security Considerations

- The UDP control interface listens on all interfaces by default
- For production use, bind to localhost only: `--bot-host 127.0.0.1`
- No authentication on UDP interface - use firewall rules if needed
- Consider running in a container or VM for isolation

## Zwift Connection Reliability

The connection between gymnasticon and Zwift can be inconsistent:

- **Protocol variability**: Sometimes Bluetooth works, sometimes only ANT+ works
- **Order sensitivity**: Connection order (Power first vs Cadence first) may affect success
- **Day-to-day changes**: What works one day might not work the next
- **Both required**: Always need to connect both Power and Cadence, regardless of protocol

### Best Practices:
1. Have both Bluetooth and ANT+ available
2. If one protocol fails, immediately try the other
3. Try different connection orders if initial attempt fails
4. Keep gymnasticon running while switching between protocols in Zwift
5. Accept that some days require more patience than others
