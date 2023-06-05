#![no_std]
#![no_main]

use esp_backtrace as _;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    loop {}
}
