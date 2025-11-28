use crate::register_tools::{self, write_bits};
use crate::registers::nvic::*;

const NVIC_ISER_REGISTERS: [*mut u32; 5] = [ISER0, ISER1, ISER2, ISER3, ISER4];

const NVIC_IPR_REGISTERS: [*mut u32; 40] = [
    IPR0, IPR1, IPR2, IPR3, IPR4, IPR5, IPR6, IPR7, IPR8, IPR9, IPR10, IPR11, IPR12, IPR13, IPR14,
    IPR15, IPR16, IPR17, IPR18, IPR19, IPR20, IPR21, IPR22, IPR23, IPR24, IPR25, IPR26, IPR27,
    IPR28, IPR29, IPR30, IPR31, IPR32, IPR33, IPR34, IPR35, IPR36, IPR37, IPR38, IPR39,
];

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum IRQLevel {
    Level0 = 0x00,
    Level1 = 0x01,
    Level2 = 0x02,
    Level3 = 0x03,
    Level4 = 0x04,
    Level5 = 0x05,
    Level6 = 0x06,
    Level7 = 0x07,
    Level8 = 0x08,
    Level9 = 0x09,
    Level10 = 0x0A,
    Level11 = 0x0B,
    Level12 = 0x0C,
    Level13 = 0x0D,
    Level14 = 0x0E,
    Level15 = 0x0F,
}

/// Enable an interrupt based on an interrupt ID from the registers::irq list
pub fn enable_interrupt(interrupt: u32) {
    unsafe {
        register_tools::enable_interrupt(interrupt, &NVIC_ISER_REGISTERS);
    }
}

/// Disable an interrupt based on an interrupt ID from the registers::irq list
pub fn disable_interrupt(interrupt: u32) {
    unsafe {
        register_tools::disable_interrupt(interrupt, &NVIC_ISER_REGISTERS);
    }
}

/// Set an irq level for an interrupt ID from the registers::irq list. Note that lower numbers have
/// higher priorities
pub fn set_irq_level(irq_id: u32, level: IRQLevel) {
    let index = (irq_id as usize) / 4;
    let field = (irq_id % 4) as u8 * 4;

    // The interrupt level is the highest 4 bits in the 8 bit field
    let level = level as u8;
    let level = (level << 4) as u32;

    unsafe { write_bits(NVIC_IPR_REGISTERS[index], field, level, 0xFF) };
}
