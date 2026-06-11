#![no_std]
#![no_main]

use core::arch::asm;

#[unsafe(no_mangle)]
fn main() {
    let msg = b"Hello, World!";
    for &c in msg {
        unsafe {
            asm!(
                // BIOS interrupt 0x10 (video services).
                // Function code 0x0E means "print a character like a teletype".

                "mov ah, 0x0E",   // ah = the function selector that int 0x10 reads
                "mov al, {0}",    // al = the current character to print
                "int 0x10",       // hand control to the BIOS so it does the printing

                // {0} = first operand listed below.
                // The compiler picks a byte-sized register (like al, bl, cl, dl)
                // and replaces {0} with that register name.
                in(reg_byte) c,

                // Mark the full AX register as clobbered.
                // This tells the compiler: (1) don't assume anything survives in AX,
                // and (2) don't pick AH or AL for the input above, since we're writing
                // to them manually and that would corrupt the input.
                out("ax") _,
            );
        }
    }
    unsafe {
        asm!("hlt"); // Halt the CPU until the next hardware interrupt
    }
}

#[panic_handler]
pub fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
