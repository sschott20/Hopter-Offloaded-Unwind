#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(naked_functions)]

extern crate alloc;
use core::sync::atomic::{AtomicUsize, Ordering};
use hadusos::Session;
// use hopter::time::sleep_ms;
use hopter::{
    debug::semihosting::{self, dbg_println},
    task::{self, main},
    time::sleep_ms,
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

    // Start a task running the `will_panic` function.
    // The task is restartable. When the panic occurs, the task's stack will be
    // unwound, and the task will be restarted.
    task::build()
        .set_entry(will_panic)
        .spawn_restartable()
        .unwrap();
}

fn will_panic() {
    // A persistent counter.
    static CNT: AtomicUsize = AtomicUsize::new(0);

    // Every time the task runs we increment it by 1.
    let cnt = CNT.fetch_add(1, Ordering::SeqCst);

    dbg_println!("Current count: {}", cnt);

    // Panic and get restarted for 5 times.
    if cnt == 0 {
        dbg_println!("Panic now!");
        panic!();
    }
    let _ = sleep_ms(120000);
    dbg_println!("Finished");

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
