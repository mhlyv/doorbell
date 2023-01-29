# Raspberry pi doorbell

This is a very quick and dirty project made just for fun. A couple of wires and tape is used to detect when the door is opened, then a randomly selected sound is played on a speaker.

## Installation

1. Add your sounds to the `sounds` directory
2. `./install.sh` (This enables the `doorbell.service` systemd service)

## Dependencies

```sudo apt install build-essential pkg-config libasound2-dev```
