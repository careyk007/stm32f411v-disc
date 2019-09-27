# `STM32F411-DISCO Quickstart`

> A template for building applications for the STM32F411-DISCO board.

This project is based on the work of [`cortex-m-quickstart`](https://github.com/rust-embedded/cortex-m-quickstart).

## Dependencies

To build embedded programs using this template you'll need (I am using):
 - Rust 1.39.0-nightly

To install the required targets, run

``` console
$ rustup target add thumbv7em-none-eabihf
```

This adds support for building for the Cortex-M4F processor

## Using this template

1. Instantiate the template.

``` console
$ git clone https://github.com/careyk007/stm32f411v-disc <directory_name>
$ cd <directory_name>
```

2. Build the template application or one of the examples.

``` console
$ cargo build
```

# TODO

## Things still left to implement:

 - User button interrept
    - `PA0`

 - DAC Output (Audio jack)
    - `CS43L22`

 - MEMS Microphone input
    - `MP45DT02`

## Things to clean up:

  - LED interface
    - Remove LEDController since it's not board-specific?

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
