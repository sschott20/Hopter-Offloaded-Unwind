// #![feature(naked_functions)]
extern crate alloc;

// use crate::interrupt::handler;

use crate::{sync::Mailbox, time::get_tick};
use core::cmp::max;
use hadusos::{Serial, SerialError, Session, Timer};
use stm32f4xx_hal::{
    pac::USART1,
    prelude::*,
    serial::{Rx, Tx},
};

pub static mut G_UART_SESSION: Option<Session<UsartSerial, UsartTimer, 150, 2>> = None;

pub static G_UART_MAILBOX: Mailbox = Mailbox::new();
pub static mut G_UART_RX: Option<Rx<USART1>> = None;
pub static mut G_UART_MAX_SIZE: usize = 0;
pub static mut G_UART_RBYTE: heapless::Deque<u8, 128> = heapless::Deque::new();
pub static G_TIMEOUT_MS: u32 = 30000;
#[derive(Debug)]
pub enum UartError {
    ReadError,
    WriteError,
    Uninitialized,
    Timeout,
}

pub struct UsartTimer {}

impl Timer for UsartTimer {
    fn get_timestamp_ms(&mut self) -> u32 {
        let tick = get_tick();
        tick
    }
}

pub struct UsartSerial {
    pub tx: Tx<USART1>,
}
impl Serial for UsartSerial {
    type ReadError = UartError;
    type WriteError = UartError;

    fn read_byte_with_timeout(
        &mut self,
        timeout_ms: u32,
    ) -> Result<u8, SerialError<Self::ReadError, Self::WriteError>> {
        let result = G_UART_MAILBOX.wait_until_timeout(timeout_ms);
        if result {
            let byte = unsafe { G_UART_RBYTE.pop_front().unwrap() };
            unsafe { G_UART_MAX_SIZE = max(G_UART_RBYTE.len(), G_UART_MAX_SIZE) };
            Ok(byte)
        } else {
            Err(SerialError::Timeout)
        }
    }
    fn write_byte(
        &mut self,
        byte: u8,
    ) -> Result<(), SerialError<Self::ReadError, Self::WriteError>> {
        self.tx.write(byte).unwrap();
        Ok(())
    }
}
