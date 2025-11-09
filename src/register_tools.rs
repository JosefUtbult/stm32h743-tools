use core::assert;
use core::ptr::{read_volatile, write_volatile};

/// Write a byte into a register
#[inline(always)]
pub unsafe fn write_register(register: *mut u32, value: u32) {
    unsafe { write_volatile(register, value) };
}

/// Read a byte from a register
#[inline(always)]
pub unsafe fn read_register(register: *const u32) -> u32 {
    unsafe { read_volatile(register) }
}

/// Set a bit in a register using `register | (1 << field)`
#[inline(always)]
pub unsafe fn set_bit(register: *mut u32, field: u8) {
    unsafe { write_volatile(register, read_volatile(register) | (1 << field)) };
}

/// Clear a bit in a register using `register & !(1 << field)`
#[inline(always)]
pub unsafe fn clear_bit(register: *mut u32, field: u8) {
    unsafe { write_volatile(register, read_volatile(register) & !(1 << field)) };
}

/// Reads a single bit from a register at `field` position.
/// Equivalent to `(register >> field) & 1`
#[inline(always)]
pub unsafe fn get_bit(register: *const u32, field: u8) -> u32 {
    (unsafe { read_volatile(register) } >> field) & 0b1
}

/// Toggle a bit in a register using `register ^ (1 << field)`
#[inline(always)]
pub unsafe fn toggle_bit(register: *mut u32, field: u8) {
    unsafe { write_volatile(register, read_volatile(register) ^ (1 << field)) };
}

/// Writes a list of bits to a register by masking out the bit positions and then inserts the value.
/// Equivalent to `(register & !(mask << field)) | (value << field)`
#[inline(always)]
pub unsafe fn write_bits(register: *mut u32, field: u8, value: u32, mask: u32) {
    unsafe {
        write_volatile(
            register,
            (read_volatile(register) & !(mask << field)) | (value << field),
        )
    };
}

pub unsafe fn set_bit_in_array(id: u32, registers: &[*mut u32]) {
    let register_index = id as usize / 32;
    let field = (id % 32) as u8;
    assert!(register_index < registers.len());

    unsafe { set_bit(registers[register_index], field) };
}

pub unsafe fn clear_bit_in_array(id: u32, registers: &[*mut u32]) {
    let register_index = id as usize / 32;
    let field = (id % 32) as u8;
    assert!(register_index < registers.len());

    unsafe { clear_bit(registers[register_index], field) };
}

pub unsafe fn write_bits_in_array(id: u32, value: u32, mask: u32, registers: &[*mut u32]) {
    let register_index = id as usize / 32;
    let field = (id % 32) as u8;
    assert!(register_index < registers.len());

    unsafe { write_bits(registers[register_index], field, value, mask) };
}

pub unsafe fn get_bits_in_array(id: u32, mask: u32, registers: &[*mut u32]) -> u32 {
    let register_index = id as usize / 32;
    let field = (id % 32) as u8;

    assert!(register_index < registers.len());
    let value = unsafe { read_volatile(registers[register_index]) };
    value & (mask << field)
}

/// Enable an interrupt in a list of interrupt registers
pub unsafe fn enable_interrupt(id: u32, interrupt_registers: &[*mut u32]) {
    unsafe { set_bit_in_array(id, interrupt_registers) };
}

/// Disable an interrupt in a list of interrupt registers
pub unsafe fn disable_interrupt(id: u32, interrupt_registers: &[*mut u32]) {
    unsafe { clear_bit_in_array(id, interrupt_registers) };
}
