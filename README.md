# Keyspeed - a typing speed tracker for i3blocks/waybar
This is a reimplementation of my [original tracker](https://github.com/indeedhat/keyspeed) this time in rust.  
It has been a simple project to learn the very basics of rust and the code is more than likely total garbage.

```
keyspeed-rs -h
A simple tool to track your typing speed. Designed to be used in conjunction with waybar

Usage: keyspeed-rs [OPTIONS] <DEVICE>

Arguments:
  <DEVICE>  manually set the keyboard device to track

Options:
  -c, --cpm                  Count CPM instead of WPM
  -b, --best                 Show best score
  -p, --pad <PAD>            Pad values with leeding 0's [default: 2]
  -i, --interval <INTERVAL>  Set the interval at wich readings are taken (in seconds) [default: 5]
  -h, --help                 Print help
  -V, --version              Print version
```
