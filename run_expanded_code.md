# How to expand RTIC code

To expand the example basic_hardware_idle.rs to the output file run following:

```Bash
cargo expand --example basic_hardware_idle > example/output.rs
```

To run expanded code, you will need to remove everything above app and insert this instead.

```Rust
#![feature(prelude_import,panic_internals,rustc_private,const_fmt_arguments_new)]
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

Then it need to be executed in rust nightly. 
It's easiest done by adding '+nightly' to the run command.


```Bash
cargo +nightly run --example output
```

