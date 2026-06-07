#![no_std]
#![no_main]

use core::arch::asm;

#[unsafe(no_mangle)]
fn main() {
    let msg = b"Hello, World!";
    for &c in msg {
        unsafe {
            asm!(
                "mov ah, 0x0E",
                "mov al, {0}",
                "int 0x10",
                in(reg_byte) c,
                out("ax") _,
            );
        }
    }
    unsafe {
        asm!("hlt");
    }
}

#[panic_handler]
pub fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
