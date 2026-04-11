#![no_std]
#![no_main]

use core::panic::PanicInfo;
use blog_os::{QemuExitCode, exit_qemu, serial_println};
use blog_os::serial_print;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Run the single test on _start
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

// The panic handler only runs 1 test successfully before exiting, so we can have only one test case.
// Also, no #[test_case] since we're running this as a regular function.
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Since the tests that panic are successful, the panic handler reports success
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
