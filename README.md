# Interception Plugin Development Library Rust 
This library aims to help you build an interception plugin in Rust.

The Interception Tools is a small set of utilities for operating on input events of evdev devices.  [...Read More](https://gitlab.com/interception/linux/tools)

## Quickstart
### Interception Mode
To run with interception use the following command
Stop the interception daemon service first

```bash
# in your case replace the <devnode> with your keyboard path that looks something like this /dev/input/by-path/platform-i8042-serio-0-event-kbd
export DEVNODE=<devnode>
sudo intercept -g $DEVNODE | sudo -E cargo run --example caps2esc | sudo uinput -d $DEVNODE
```
The above example will convert the caps lock key to escape key

### Debug Mode
To view the events in debug mode 
Stop the interception daemon service first

```bash
sudo systemctl stop udevmon.service 
```
Then run 

```bash
export DEVNODE=<devnode>
cargo run --example caps2esc $DEVNODE
```

This will show output which events are being written to stdout
```
MISC
KEY_ESC "Down"
SYNCHRONIZATION
LED
LED
SYNCHRONIZATION
MISC
KEY_ESC "Up"
SYNCHRONIZATION
MISC
KEY_ESC "Down"
SYNCHRONIZATION
MISC
KEY_ESC "Up"
SYNCHRONIZATION
LED
SYNCHRONIZATION
MISC
KEY_ESC "Down"
SYNCHRONIZATION
LED
SYNCHRONIZATION
MISC
KEY_ESC "Up"
SYNCHRONIZATION
```

#### Credits:
- [evdev](https://github.com/emberian/evdev)
