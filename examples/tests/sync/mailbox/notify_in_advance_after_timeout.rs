//! Test in-advance notification on mailbox after a timeout on the mailbox.

#![no_std]
#![no_main]

extern crate alloc;
use hopter::{
    debug::semihosting::{self, dbg_println},
    sync::Mailbox,
    task,
    task::main,
};

static MAILBOX: Mailbox = Mailbox::new();

#[main]
fn main(_: cortex_m::Peripherals) {
    task::build().set_entry(listener).spawn().unwrap();
}

fn listener() {
    let notified = MAILBOX.wait_until_timeout(500);
    if notified {
        dbg_println!("Unexpected notification.");
        #[cfg(feature = "qemu")]
        semihosting::terminate(false);
        #[cfg(not(feature = "qemu"))]
        {
            dbg_println!("test complete!");
            loop {}
        }
    }

    MAILBOX.notify_allow_isr();

    let notified = MAILBOX.wait_until_timeout(500);
    if !notified {
        dbg_println!("Unexpected timeout.");
        #[cfg(feature = "qemu")]
        semihosting::terminate(false);
        #[cfg(not(feature = "qemu"))]
        {
            dbg_println!("test complete!");
            loop {}
        }
    }

    #[cfg(feature = "qemu")]
    semihosting::terminate(true);
    #[cfg(not(feature = "qemu"))]
    {
        dbg_println!("test complete!");
        loop {}
    }
}
