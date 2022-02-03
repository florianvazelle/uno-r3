# uno r3

## Setup

```
rustup toolchain install nightly-2021-01-07
rustup +nightly-2021-01-07 component add rust-src
```

If you get a permission denied error when accessing `/dev/ttyACM0`. You may need to add your user to a linux user group that has access to the serial interface. First, we find the group using:

```
ls -l /dev/ttyACM0
```

I get the following output on my machine:

```
crw-rw---- 1 root uucp 166, 0 Aug 19 03:29 /dev/ttyACM0
```

To add your username to uucp group, we run:

```
sudo usermod -a -G uucp $USER
```

## Building

### Build and run a standalone target

Use the following command to build and run the first executable target.
```
sudo -u $USER cargo run --bin 1-blink
```

## References

- [Tutorial Rust on Arduino Uno](https://creativcoder.dev/rust-on-arduino-uno)
- [Rust avr-hal Crates](https://github.com/Rahix/avr-hal)
