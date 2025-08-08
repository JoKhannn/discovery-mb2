#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use microbit as _;
use panic_halt as _;
use rtt_target::{rtt_init_print, rprintln}; 

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut i = 0;
    loop {
        rprintln!("Hello! {}", i);
        i += 1;
        for _ in 0..100_000{
            asm::nop();
        }
    }
}
