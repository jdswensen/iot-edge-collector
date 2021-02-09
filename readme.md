# iot-edge-collector
A simple example of IoT data collection on a Linux based IoT edge device.

## Requirements
- Raspberry Pi
- [Sense HAT](https://www.raspberrypi.org/products/sense-hat/)
- Raspberry Pi OS
    Ubuntu could work but requires more configuration. Most things are ready to go out of the box with RPiOS.
- InfluxDB 2.0 setup locally or in the cloud
    The cloud interface is probably the easiest to get started with.

## Setup
For now, this project requires local compilation on the Raspberry Pi.

```
# Make sure everything is up to date
sudo apt update
sudo apt upgrade

# Install dependancies
sudo apt install -y build-essential libssl-dev pkg-config i2c-tools

# Enable I2C devices
sudo raspi-config
# Select:
# Interface Options
# I2C
# Yes

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone project
git clone https://github.com/jdswensen/iot-edge-collector.git
```

## Usage
Create a JSON [config file](/test/endpoint-cfg.json) containing your endpoint configuration. All of these values are available through the InfluxDB cloud.

Once the configuration is set, the program can be run with
```
cargo run -- --config ./path/to/config.json
```