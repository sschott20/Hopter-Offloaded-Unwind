//! Hopter is a Rust-based embedded operating system built to enable memory-safe,
//! efficient, reliable, and responsive applications. It is co-designed with a
//! [customized compiler](https://github.com/hopter-project/hopter-compiler-toolchain)
//! that guarantees additional invariants beyond what Rust can express. However,
//! the changes to the compiler are transparent to application programmers, and
//! Rust's syntax remains unchanged.
//!
//! Hopter does not rely on any hardware protection mechanisms, providing
//! safety purely through software. However, it does not anticipate malicious
//! applications. The threat model is similar to that assumed by FreeRTOS.
//!
//! # Getting Started
//!
//! To get started with Hopter, check out our
//! [quick start guide](https://github.com/hopter-project/hopter-quick-start)
//! that provides the instructions to set up the environment and an
//! introduction to Hopter's API.
//!
//! # Feature Overview
//!
//! ## Memory Safety
//!
//! Hopter prevents stack overflows on top of other memory safety aspects
//! guaranteed by Rust. The customized compiler generates an additional
//! prologue for every compiled function. If the prologue detects an impending
//! overflow, it diverts the control flow to the kernel, which will in turn
//! decide whether to extend the stack or to terminate the task and reclaim
//! its resources.
//!
//! ## Memory Efficiency
//!
//! Hopter can allocate stacks on-demand in small chunks called stacklets,
//! time-multiplexing the stack memory among tasks. The technique is known as
//! the segmented stack. When function calls occur, the stack may be extended
//! by allocating more stacklets, which will subsequently be freed when the
//! functions return.
//!
//! Hopter further provides the breathing task API to better facilitate
//! time-multiplexing the stack memory, and also alleviates the performance
//! drop due to segmented stack hot-split.
//!
//! ## Reliability
//!
//! Hopter is not afraid of panic. The stack unwinder cleans up the panicking
//! task or IRQ handler’s stack by calling the drop handlers of all live
//! objects, ensuring that resources are properly released. Tasks can be
//! spwaned as restartable tasks, which automatically restart if they panic.
//!
//! Hopter also uses the stack unwinder to terminate tasks that exceed their
//! stack size limit, with the customized compiler assisting in avoiding corner
//! cases where unwinding starts within a drop handler.
//!
//! ## Responsiveness
//!
//! Hopter supports zero-latency IRQ handling. The kernel never disables IRQs,
//! not even in the parts that are traditionally considered as critical
//! sections. This ensures that pending interrupts are handled immediately.
//! A novel synchronization primitive, called soft-lock, manages concurrent
//! access between IRQs and tasks without the need to disable IRQs.

#![no_std]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(lang_items)]
#![feature(negative_impls)]
#![allow(internal_features)]
#![feature(new_uninit)]
#![feature(raw_ref_op)]
#![feature(alloc_error_handler)]

extern crate alloc;

mod allocator;
mod assembly;
mod boot;
mod schedule;
mod unrecoverable;
mod unwind;

pub mod config;
pub mod debug;
pub mod interrupt;
pub mod sync;
pub mod task;
pub mod time;
pub mod uart;