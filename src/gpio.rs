/// See [RM0433 Reference Manual](https://www.st.com/resource/en/reference_manual/rm0433-stm32h742-stm32h743753-and-stm32h750-value-line-advanced-armbased-32bit-mcus-stmicroelectronics.pdf)
use super::{
    register_tools::{clear_bit, get_bit, set_bit, toggle_bit, write_bits},
    registers,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GpioRegister {
    GpioA,
    GpioB,
    GpioC,
    GpioD,
    GpioE,
    GpioH,
    GpioI,
    GpioJ,
    GpioK,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GpioPin {
    P0 = 0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GpioMode {
    Input = 0b00,
    Output = 0b01,
    Alternate = 0b10,
    Analog = 0b11,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GpioPull {
    NoPull = 0b00,
    PullDown = 0b01,
    PullUp = 0b10,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GpioOutputMode {
    PushPull = 0b0,
    OpenDrain = 0b1,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GpioSpeed {
    LowSpeed,
    MediumSpeed,
    HighSpeed,
    VeryHighSpeed,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GpioAlternate {
    AF0 = 0b0000,
    AF1 = 0b0001,
    AF2 = 0b0010,
    AF3 = 0b0011,
    AF4 = 0b0100,
    AF5 = 0b0101,
    AF6 = 0b0110,
    AF7 = 0b0111,
    AF8 = 0b1000,
    AF9 = 0b1001,
    AF10 = 0b1010,
    AF11 = 0b1011,
    AF12 = 0b1100,
    AF13 = 0b1101,
    AF14 = 0b1110,
    AF15 = 0b1111,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gpio {
    pub register: GpioRegister,
    pub pin: GpioPin,
    pub mode: GpioMode,
    pub output_mode: GpioOutputMode,
    pub pull: GpioPull,
    pub speed: GpioSpeed,
    pub alternate: GpioAlternate,
}

impl Gpio {
    pub const fn new() -> Self {
        Self {
            register: GpioRegister::GpioA,
            pin: GpioPin::P0,
            mode: GpioMode::Input,
            output_mode: GpioOutputMode::PushPull,
            pull: GpioPull::NoPull,
            speed: GpioSpeed::LowSpeed,
            alternate: GpioAlternate::AF0,
        }
    }

    pub fn setup(&self) {
        use registers::{
            gpioa, gpiob, gpioc, gpiod, gpioe, gpioh, gpioi, gpioj, gpiok,
            rcc::{AHB4ENR, ahb4enr},
        };

        // Enable the gpio clock in ahb1
        let ahb1_en_field = match self.register {
            GpioRegister::GpioA => ahb4enr::GPIOAEN,
            GpioRegister::GpioB => ahb4enr::GPIOBEN,
            GpioRegister::GpioC => ahb4enr::GPIOCEN,
            GpioRegister::GpioD => ahb4enr::GPIODEN,
            GpioRegister::GpioE => ahb4enr::GPIOEEN,
            GpioRegister::GpioH => ahb4enr::GPIOHEN,
            GpioRegister::GpioI => ahb4enr::GPIOIEN,
            GpioRegister::GpioJ => ahb4enr::GPIOJEN,
            GpioRegister::GpioK => ahb4enr::GPIOKEN,
        };

        // See section 6.3.9 RCC AHB1 peripheral clock enable register (RCC_ahb4enr)
        unsafe {
            set_bit(AHB4ENR, ahb1_en_field);
        }

        let moder_register = match self.register {
            GpioRegister::GpioA => gpioa::MODER,
            GpioRegister::GpioB => gpiob::MODER,
            GpioRegister::GpioC => gpioc::MODER,
            GpioRegister::GpioD => gpiod::MODER,
            GpioRegister::GpioE => gpioe::MODER,
            GpioRegister::GpioH => gpioh::MODER,
            GpioRegister::GpioI => gpioi::MODER,
            GpioRegister::GpioJ => gpioj::MODER,
            GpioRegister::GpioK => gpiok::MODER,
        };

        unsafe {
            // Clear and write the general pin mode to the MODER register
            write_bits(moder_register, self.pin as u8 * 2, self.mode as u32, 0b11);
        }

        let otyper_register = match self.register {
            GpioRegister::GpioA => gpioa::OTYPER,
            GpioRegister::GpioB => gpiob::OTYPER,
            GpioRegister::GpioC => gpioc::OTYPER,
            GpioRegister::GpioD => gpiod::OTYPER,
            GpioRegister::GpioE => gpioe::OTYPER,
            GpioRegister::GpioH => gpioh::OTYPER,
            GpioRegister::GpioI => gpioi::OTYPER,
            GpioRegister::GpioJ => gpioj::OTYPER,
            GpioRegister::GpioK => gpiok::OTYPER,
        };

        if self.output_mode == GpioOutputMode::PushPull {
            unsafe {
                // Set the OTYPER register to 0b0, to enable output push-pull
                clear_bit(otyper_register, self.pin as u8);
            }
        } else {
            unsafe {
                // Set the OTYPER register to 0b1, to enable floating output
                set_bit(otyper_register, self.pin as u8);
            }
        }

        let pupdr_register = match self.register {
            GpioRegister::GpioA => gpioa::PUPDR,
            GpioRegister::GpioB => gpiob::PUPDR,
            GpioRegister::GpioC => gpioc::PUPDR,
            GpioRegister::GpioD => gpiod::PUPDR,
            GpioRegister::GpioE => gpioe::PUPDR,
            GpioRegister::GpioH => gpioh::PUPDR,
            GpioRegister::GpioI => gpioi::PUPDR,
            GpioRegister::GpioJ => gpioj::PUPDR,
            GpioRegister::GpioK => gpiok::PUPDR,
        };

        unsafe {
            // Set the PUPDR register to enable/disable pull up/down
            write_bits(pupdr_register, self.pin as u8, self.pull as u32, 0b11);
        }

        if self.mode == GpioMode::Alternate {
            let afr_register = if self.pin < GpioPin::P8 {
                match self.register {
                    GpioRegister::GpioA => gpioa::AFRL,
                    GpioRegister::GpioB => gpiob::AFRL,
                    GpioRegister::GpioC => gpioc::AFRL,
                    GpioRegister::GpioD => gpiod::AFRL,
                    GpioRegister::GpioE => gpioe::AFRL,
                    GpioRegister::GpioH => gpioh::AFRL,
                    GpioRegister::GpioI => gpioi::AFRL,
                    GpioRegister::GpioJ => gpioj::AFRL,
                    GpioRegister::GpioK => gpiok::AFRL,
                }
            } else {
                match self.register {
                    GpioRegister::GpioA => gpioa::AFRH,
                    GpioRegister::GpioB => gpiob::AFRH,
                    GpioRegister::GpioC => gpioc::AFRH,
                    GpioRegister::GpioD => gpiod::AFRH,
                    GpioRegister::GpioE => gpioe::AFRH,
                    GpioRegister::GpioH => gpioh::AFRH,
                    GpioRegister::GpioI => gpioi::AFRH,
                    GpioRegister::GpioJ => gpioj::AFRH,
                    GpioRegister::GpioK => gpiok::AFRH,
                }
            };

            // Set the alternate function for the pin in either the AFR high or low register
            let afr_field = (self.pin as u8 % 8) * 4;

            unsafe {
                write_bits(afr_register, afr_field, self.alternate as u32, 0b1111);
            }
        }
    }

    pub fn set(&self) {
        set(self.register, self.pin);
    }

    pub fn get(&self) -> bool {
        get(self.register, self.pin)
    }

    pub fn clear(&self) {
        clear(self.register, self.pin);
    }

    pub fn toggle(&self) {
        toggle(self.register, self.pin);
    }
}

fn set(register: GpioRegister, pin: GpioPin) {
    let odr = get_odr(register, pin);
    unsafe {
        set_bit(odr.0, odr.1);
    }
}

fn clear(register: GpioRegister, pin: GpioPin) {
    let odr = get_odr(register, pin);
    unsafe {
        clear_bit(odr.0, odr.1);
    }
}

fn toggle(register: GpioRegister, pin: GpioPin) {
    let odr = get_odr(register, pin);
    unsafe {
        toggle_bit(odr.0, odr.1);
    }
}

fn get(register: GpioRegister, pin: GpioPin) -> bool {
    let idr = get_idr(register, pin);
    unsafe { get_bit(idr.0, idr.1) == 1 }
}

fn get_odr(register: GpioRegister, pin: GpioPin) -> (*mut u32, u8) {
    use registers::{gpioa, gpiob, gpioc, gpiod, gpioe, gpioh, gpioi, gpioj, gpiok};

    let odr_register = match register {
        GpioRegister::GpioA => gpioa::ODR,
        GpioRegister::GpioB => gpiob::ODR,
        GpioRegister::GpioC => gpioc::ODR,
        GpioRegister::GpioD => gpiod::ODR,
        GpioRegister::GpioE => gpioe::ODR,
        GpioRegister::GpioH => gpioh::ODR,
        GpioRegister::GpioI => gpioi::ODR,
        GpioRegister::GpioJ => gpioj::ODR,
        GpioRegister::GpioK => gpiok::ODR,
    };

    let odr_field = match pin {
        GpioPin::P0 => gpioa::odr::OD0,
        GpioPin::P1 => gpioa::odr::OD1,
        GpioPin::P2 => gpioa::odr::OD2,
        GpioPin::P3 => gpioa::odr::OD3,
        GpioPin::P4 => gpioa::odr::OD4,
        GpioPin::P5 => gpioa::odr::OD5,
        GpioPin::P6 => gpioa::odr::OD6,
        GpioPin::P7 => gpioa::odr::OD7,
        GpioPin::P8 => gpioa::odr::OD8,
        GpioPin::P9 => gpioa::odr::OD9,
        GpioPin::P10 => gpioa::odr::OD10,
        GpioPin::P11 => gpioa::odr::OD11,
        GpioPin::P12 => gpioa::odr::OD12,
        GpioPin::P13 => gpioa::odr::OD13,
        GpioPin::P14 => gpioa::odr::OD14,
        GpioPin::P15 => gpioa::odr::OD15,
    };

    (odr_register, odr_field)
}

const fn get_idr(register: GpioRegister, pin: GpioPin) -> (*mut u32, u8) {
    use registers::{gpioa, gpiob, gpioc, gpiod, gpioe, gpioh, gpioi, gpioj, gpiok};

    let odr_register = match register {
        GpioRegister::GpioA => gpioa::IDR,
        GpioRegister::GpioB => gpiob::IDR,
        GpioRegister::GpioC => gpioc::IDR,
        GpioRegister::GpioD => gpiod::IDR,
        GpioRegister::GpioE => gpioe::IDR,
        GpioRegister::GpioH => gpioh::IDR,
        GpioRegister::GpioI => gpioi::IDR,
        GpioRegister::GpioJ => gpioj::IDR,
        GpioRegister::GpioK => gpiok::IDR,
    };

    let odr_field = match pin {
        GpioPin::P0 => gpioa::idr::ID0,
        GpioPin::P1 => gpioa::idr::ID1,
        GpioPin::P2 => gpioa::idr::ID2,
        GpioPin::P3 => gpioa::idr::ID3,
        GpioPin::P4 => gpioa::idr::ID4,
        GpioPin::P5 => gpioa::idr::ID5,
        GpioPin::P6 => gpioa::idr::ID6,
        GpioPin::P7 => gpioa::idr::ID7,
        GpioPin::P8 => gpioa::idr::ID8,
        GpioPin::P9 => gpioa::idr::ID9,
        GpioPin::P10 => gpioa::idr::ID10,
        GpioPin::P11 => gpioa::idr::ID11,
        GpioPin::P12 => gpioa::idr::ID12,
        GpioPin::P13 => gpioa::idr::ID13,
        GpioPin::P14 => gpioa::idr::ID14,
        GpioPin::P15 => gpioa::idr::ID15,
    };

    (odr_register, odr_field)
}

/// Create a simple output gpio
pub const fn create_output(register: GpioRegister, pin: GpioPin) -> Gpio {
    let mut led = Gpio::new();
    led.register = register;
    led.pin = pin;
    led.mode = GpioMode::Output;
    led
}
