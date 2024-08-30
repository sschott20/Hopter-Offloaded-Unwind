#![no_main]
#![no_std]

extern crate alloc;
use hopter::{debug::semihosting::{self, dbg_println}, sync::Semaphore, task::main};

static SEMAPHORE: Semaphore = Semaphore::new(5, 3);

#[main]
fn main(_: cortex_m::Peripherals) {
    let result = SEMAPHORE.try_down_allow_isr();

    match result {
        Ok(()) => {}
        Err(()) => {
            dbg_println!("Did not decrement");
            semihosting::terminate(false);
        }
    }

    // SEMAPHORE count should be 2
    SEMAPHORE.down();
    SEMAPHORE.down();

    // SEMAPHORE count should be 0
    let second_result = SEMAPHORE.try_down_allow_isr();

    match second_result {
        Ok(()) => {
            dbg_println!("Decremented at 0");
            semihosting::terminate(false);
        }
        Err(()) => {
            dbg_println!("Test Passed");
            semihosting::terminate(true);
        }
    }
}
