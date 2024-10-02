
#![no_main]
#![no_std]

use panic_semihosting as _;

#[external_rtic_pass::generate_rtic_app]
mod app{
    fn replace_me(){
        // this function is replaced with an real rtic app  
    }
}
