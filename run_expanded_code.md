# How to expand RTIC code

To expand the example basic_hardware_idle.rs to the output file run following:

```Bash
cargo expand --example basic_hardware_idle > output
```

To run expanded code, you will need to remove everything above app and insert this instead.

```Rust
#![feature(prelude_import,core_panic,rustc_private,const_fmt_arguments_new)]
// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]
#[prelude_import]
#[macro_use]
extern crate core;
extern crate compiler_builtins;
use panic_semihosting as _;
```

You will also need to change to rust nighty `rustup default nightly` and install thumbv7m-none-eabi `rustup target add thumbv7m-none-eabi`

Switch back to to stable `rustup default stable`

I usually copy all the code from output to a.rs and then run.

```Bash
cargo run --example a
```

I just have the file a.rs because _a_ is fast to write.
