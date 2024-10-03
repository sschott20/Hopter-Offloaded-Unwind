//! Tasks blocking on a mutex should be woken up in the sequence based on
//! their priority.

#![no_main]
#![no_std]

extern crate alloc;
use hopter::{
    config,
    debug::semihosting::{self, dbg_println},
    sync::Mutex,
    task,
    task::main,
};

static MUTEX: Mutex<()> = Mutex::new(());

#[main]
fn main(_: cortex_m::Peripherals) {
    // Hold the lock at the beginning.
    let guard = MUTEX.lock();

    // Create test tasks.
    task::build()
        .set_entry(low_task)
        .set_priority(config::DEFAULT_TASK_PRIORITY + 1)
        .spawn()
        .unwrap();
    task::build()
        .set_entry(high_task)
        .set_priority(config::DEFAULT_TASK_PRIORITY - 1)
        .spawn()
        .unwrap();
    task::build()
        .set_entry(middle_task)
        .set_priority(config::DEFAULT_TASK_PRIORITY)
        .spawn()
        .unwrap();

    // Let the test tasks run. But they will be blocked by the mutex.
    task::change_current_priority(config::DEFAULT_TASK_PRIORITY + 2).unwrap();

    // Release the mutex and the test tasks should be woken up based on their
    // respective priority.
    core::mem::drop(guard);

    #[cfg(feature = "qemu")]
    semihosting::terminate(true);
    #[cfg(not(feature = "qemu"))]
    {
        dbg_println!("test complete!");
        loop {}
    }
}

fn high_task() {
    let _gaurd = MUTEX.lock();
    dbg_println!("High priority task locking data");
}

fn middle_task() {
    let _gaurd = MUTEX.lock();
    dbg_println!("Middle priority task locking data");
}

fn low_task() {
    let _gaurd = MUTEX.lock();
    dbg_println!("Low priority task locking data");
}
