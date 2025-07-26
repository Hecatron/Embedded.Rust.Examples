# Probe-RS Install

Probe-RS is a useful tool for debugging embedded devices running rust over a SWD JTag Connection

## Installation

First we could do with cargo binstall
```bash
cargo install cargo-binstall
```

Next to install probe-rs
```
cargo binstall probe-rs-tools
# Add shell completion
probe-rs complete install
```

In some cases using a more cutting edge version might fix some bugs
```bash
cargo uninstall probe-rs-tools
cargo install --git https://github.com/probe-rs/probe-rs.git --branch master probe-rs-tools
```


## Udev rules

Under Linux depending on the distro permission needs to be granted for the user to access the devices

  * https://probe.rs/docs/getting-started/probe-setup/#linux%3A-udev-rules

First we add the user account to the plugdev group
```bash
gpasswd -a <user> plugdev
```

Next to edit the udev rules (systemd / gentoo as an example)
```bash
# Download rules from probe-rs
cd /etc/udev/rules.d
wget https://probe.rs/files/69-probe-rs.rules
# To make sure the new rules are applied
udevadm control --reload
# Apply to any already attached devices
udevadm trigger
```
