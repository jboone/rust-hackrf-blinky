# rust-hackrf-blinky

Blink the HackRF LEDs using Rust

# Building

To generate a binary for the LPC4320 M4 core on the HackRF:

```
xargo build --release
```

This leaves an ELF binary at target/thumbv7em-none-eabihf/release/hackrf-blinky

You can strip the ELF down to just the binary portion like this:

```
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/hackrf-blinky hackrf-blinky.bin
```

This can then be programmed onto your HackRF by entering DFU mode and executing these two commands:

```
dfu-util --device 1fc9:000c --download --reset
hackrf_spiflash -w hackrf-blinky.bin
```

...and then resetting the HackRF.

## License

This project is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), with portions covered by
various BSD-like licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

## Contributing

[IRC] is the dominant form of communication in this project. Please join
us on [freenode] at [#portapack].

[IRC]: https://en.wikipedia.org/wiki/Internet_Relay_Chat
[freenode]: https://freenode.net/
[#portapack]: https://webchat.freenode.net/?channels=%23portapack&uio=d4

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
