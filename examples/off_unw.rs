#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(naked_functions)]
#![allow(warnings)]
extern crate alloc;
use core::sync::atomic::{AtomicUsize, Ordering};
use hadusos::Session;
use hopter::unwind::unw_catch::catch_unwind;

use hopter::{
    debug::semihosting::{self, dbg_println},
    task::{self, main},
    time::{get_tick, sleep_ms},
    uart::{UsartSerial, UsartTimer, G_UART_MAILBOX, G_UART_RBYTE, G_UART_RX, G_UART_SESSION},
};
use hopter_proc_macro::handler;
use stm32f4xx_hal::serial::{Rx, Tx};
use stm32f4xx_hal::uart::Config;
use stm32f4xx_hal::{pac::USART1, prelude::*};

// Attribute `#[main]` marks the function as the entry function for the main
// task. The function name can be arbitrary. The main function should accept
// one argument which is the Cortex-M core peripherals.
#[main]
fn main(_: cortex_m::Peripherals) {
    dbg_println!("Beginning unw_iter example: Initializing global hadusos session");

    let t = get_tick();

    // Initialize the hadusos Session with the UART peripheral.
    // First we acquire the peripherals for the tx and rx pins
    let dp = unsafe { stm32f4xx_hal::pac::Peripherals::steal() };
    let clocks = dp.RCC.constrain().cfgr.freeze();
    let gpioa = dp.GPIOA.split();

    let usart1_pins = (
        gpioa.pa9.into_alternate::<7>(),
        gpioa.pa10.into_alternate::<7>(),
    );
    let mut rx: Rx<USART1>;
    let tx: Tx<USART1>;
    (tx, rx) = dp
        .USART1
        .serial(
            usart1_pins,
            Config::default().baudrate(115200.bps()),
            &clocks,
        )
        .unwrap()
        .split();

    rx.listen();

    unsafe {
        G_UART_RX = Some(rx);
    }

    unsafe { cortex_m::peripheral::NVIC::unmask(stm32f4xx_hal::pac::Interrupt::USART1) };

    let usart_serial = UsartSerial { tx };
    let usart_timer = UsartTimer {};
    let session: Session<UsartSerial, UsartTimer, 150, 2> = Session::new(usart_serial, usart_timer);

    unsafe { G_UART_SESSION = Some(session) };
    
    dbg_println!("Session initialized in {} ms", get_tick() - t);

    panic_tests(1);

    // When running with QEMU, this will cause the QEMU process to terminate.
    // Do not include this line when running with OpenOCD, because it will
    // clobber its internal states.
    #[cfg(feature = "qemu")]
    semihosting::terminate(true);

    #[cfg(not(feature = "qemu"))]
    {
        dbg_println!("test complete!");
        loop {}
    }
}

fn f() {
    panic!("panicking in f()");
}

fn panic_tests(iterations: usize) {
    let mut start = get_tick();
    for _ in 0..iterations {
        let result = catch_unwind(f);
    }
    let mut finish = get_tick();
    dbg_println!("Time to panic {} times: {} ms", iterations, finish - start);
    dbg_println!("Average time: {} ms", (finish - start) / iterations as u32);
}

#[handler(USART1)]
fn usart1_handler() {
    cortex_m::interrupt::free(|_| {
        unsafe {
            let _ = G_UART_RBYTE.push_back(G_UART_RX.as_mut().unwrap().read().unwrap());
        };
        // Notify the mailbox that a byte is available to read by incrementing the counter
        G_UART_MAILBOX.notify_allow_isr();
    });
}
