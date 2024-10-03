//! Test waiting with timeout but getting notified before timeout.

#![no_std]
#![no_main]

extern crate alloc;
use hopter::{
    debug::semihosting::{self, dbg_println},
    sync::Mailbox,
    task,
    task::main,
    time,
};

static MAILBOX: Mailbox = Mailbox::new();

#[main]
fn main(_: cortex_m::Peripherals) {
    MAILBOX.notify_allow_isr();

    task::build()
        .set_entry(listener)
        .set_priority(4)
        .spawn()
        .unwrap();

    task::build()
        .set_entry(notifier)
        .set_priority(8)
        .spawn()
        .unwrap();
}

fn listener() {
    let notified = MAILBOX.wait_until_timeout(1000);
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

    let notified = MAILBOX.wait_until_timeout(1000);
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

fn notifier() {
    time::sleep_ms(500);
    MAILBOX.notify_allow_isr();
}
