use super::register_tools::{clear_bit, get_bit, set_bit, write_bits, write_register};

pub enum USART {
    USART2,
    USART3,
}

fn get_cr_usart_control_register(usart: &USART) -> *mut u32 {
    use super::registers::{usart2, usart3};

    match usart {
        USART::USART2 => usart2::CR1,
        USART::USART3 => usart3::CR1,
    }
}

fn get_apb1lenr_usart_clock_enable_field(usart: &USART) -> u8 {
    use super::registers::rcc;

    match usart {
        USART::USART2 => rcc::apb1lenr::USART2EN,
        USART::USART3 => rcc::apb1lenr::USART3EN,
    }
}

fn setup_usart(clock_speed: u32, baud_rate: u32, usart: &USART) {
    use super::{
        gpio::{Gpio, GpioAlternate, GpioMode, GpioPin, GpioRegister, GpioSpeed},
        registers::{rcc, usart2, usart3},
    };

    let cr_usart_control_register = get_cr_usart_control_register(usart);
    let apb1lenr_usart_clock_enable_field = get_apb1lenr_usart_clock_enable_field(usart);
    let ahb4enr_gpio_clock_enable_field = match usart {
        USART::USART2 => rcc::ahb4enr::GPIOAEN,
        USART::USART3 => rcc::ahb4enr::GPIODEN,
    };

    let mut usart_tx_gpio = match usart {
        USART::USART2 => {
            let mut gpio = Gpio::new();
            gpio.register = GpioRegister::GpioA;
            gpio.pin = GpioPin::P2;
            gpio
        }
        USART::USART3 => {
            let mut gpio = Gpio::new();
            gpio.register = GpioRegister::GpioD;
            gpio.pin = GpioPin::P8;
            gpio
        }
    };

    let mut usart_rx_gpio = match usart {
        USART::USART2 => {
            let mut gpio = Gpio::new();
            gpio.register = GpioRegister::GpioA;
            gpio.pin = GpioPin::P3;
            gpio
        }
        USART::USART3 => {
            let mut gpio = Gpio::new();
            gpio.register = GpioRegister::GpioD;
            gpio.pin = GpioPin::P9;
            gpio
        }
    };

    usart_tx_gpio.mode = GpioMode::Alternate;
    usart_tx_gpio.speed = GpioSpeed::HighSpeed;
    usart_tx_gpio.alternate = GpioAlternate::AF7;

    usart_rx_gpio.mode = GpioMode::Alternate;
    usart_rx_gpio.speed = GpioSpeed::HighSpeed;
    usart_rx_gpio.alternate = GpioAlternate::AF7;

    let brr_usart_baud_rate_register = match usart {
        USART::USART2 => usart2::BRR,
        USART::USART3 => usart3::BRR,
    };

    unsafe {
        // Disable USART before configuring
        clear_bit(cr_usart_control_register, usart3::cr1::UE);

        // Enable the usart clock
        set_bit(rcc::APB1LENR, apb1lenr_usart_clock_enable_field);

        // Enable the gpioa clock
        set_bit(rcc::AHB4ENR, ahb4enr_gpio_clock_enable_field);

        // Setup gpio pins as alternate functions (usart)
        usart_tx_gpio.setup();
        usart_rx_gpio.setup();

        // From section 48.5.7 USART baud rate generation
        let usartdiv = clock_speed as f32 / (baud_rate << 4) as f32;
        let mantissa = usartdiv as u32;
        let fraction = ((usartdiv - mantissa as f32) * 16.0) as u32;

        // Set the baud rate
        write_bits(
            brr_usart_baud_rate_register,
            usart3::brr::BRR_4_15,
            mantissa,
            0xfff,
        );
        write_bits(
            brr_usart_baud_rate_register,
            usart3::brr::BRR_0_3,
            fraction,
            0xf,
        );

        // Enable transmit
        set_bit(cr_usart_control_register, usart3::cr1::TE);

        // Enable usart3
        set_bit(cr_usart_control_register, usart3::cr1::UE);
    }
}

pub fn cleanup_usart(usart: &USART) {
    use super::registers::{rcc, usart2};

    let apb1lenr_usart_clock_enable_field = get_apb1lenr_usart_clock_enable_field(usart);
    let cr_usart_control_register = get_cr_usart_control_register(usart);

    unsafe {
        // Disable the usart clock
        clear_bit(rcc::APB1LENR, apb1lenr_usart_clock_enable_field);

        // Disable transmit
        clear_bit(cr_usart_control_register, usart2::cr1::TE);

        // Disable usart2
        clear_bit(cr_usart_control_register, usart2::cr1::UE);
    }
}

pub fn is_usart_setup(usart: &USART) -> bool {
    use super::registers::usart2;
    let cr_usart_control_register = get_cr_usart_control_register(usart);

    unsafe {
        let transmit_enable = get_bit(cr_usart_control_register, usart2::cr1::TE) == 1;
        let usart_enable = get_bit(cr_usart_control_register, usart2::cr1::UE) == 1;
        transmit_enable && usart_enable
    }
}

pub fn enable_usart_tx_interrupt(usart: &USART) {
    use super::registers::usart2;
    let cr_usart_control_register = get_cr_usart_control_register(usart);

    unsafe {
        // Enable the transmit interrupt
        set_bit(cr_usart_control_register, usart2::cr1::TXEIE);
    }
}

pub fn disable_usart_tx_interrupt(usart: &USART) {
    use super::registers::usart2;
    let cr_usart_control_register = get_cr_usart_control_register(usart);

    unsafe {
        // Disable the transmit interrupt
        clear_bit(cr_usart_control_register, usart2::cr1::TXEIE);
    }
}

pub fn write_usart_character(character: char, usart: &USART) {
    use super::registers::{usart2, usart3};

    if !is_usart_setup(usart) {
        return;
    }

    let isr_usart_interrupt_register = match usart {
        USART::USART2 => usart2::ISR,
        USART::USART3 => usart3::ISR,
    };

    let tdr_usart_data_register = match usart {
        USART::USART2 => usart2::TDR,
        USART::USART3 => usart3::TDR,
    };

    unsafe {
        // Ensure USART TX buffer is ready
        while get_bit(isr_usart_interrupt_register, usart2::isr::TXE) == 0 {}

        // Write the character to the USART Data Register
        write_register(tdr_usart_data_register, character as u32);
    }
}

pub fn write_usart_string(string: &str, usart: &USART) {
    if !is_usart_setup(usart) {
        return;
    }

    for character in string.chars() {
        write_usart_character(character, usart);
    }
}

// USART 2

pub fn setup_usart2(clock_speed: u32, baud_rate: u32) {
    setup_usart(clock_speed, baud_rate, &USART::USART2);
}

pub fn cleanup_usart2() {
    cleanup_usart(&USART::USART2);
}

pub fn is_usart2_setup() -> bool {
    is_usart_setup(&USART::USART2)
}

pub fn enable_usart2_tx_interrupt() {
    enable_usart_tx_interrupt(&USART::USART2);
}

pub fn disable_usart2_tx_interrupt() {
    disable_usart_tx_interrupt(&USART::USART2);
}

pub fn write_usart2_character(character: char) {
    write_usart_character(character, &USART::USART2);
}

pub fn write_usart2_string(string: &str) {
    write_usart_string(string, &USART::USART2);
}

// USART 3

pub fn setup_usart3(clock_speed: u32, baud_rate: u32) {
    setup_usart(clock_speed, baud_rate, &USART::USART3);
}

pub fn cleanup_usart3() {
    cleanup_usart(&USART::USART3);
}

pub fn is_usart3_setup() -> bool {
    is_usart_setup(&USART::USART3)
}

pub fn enable_usart3_tx_interrupt() {
    enable_usart_tx_interrupt(&USART::USART3);
}

pub fn disable_usart3_tx_interrupt() {
    disable_usart_tx_interrupt(&USART::USART3);
}

pub fn write_usart3_character(character: char) {
    write_usart_character(character, &USART::USART3);
}

pub fn write_usart3_string(string: &str) {
    write_usart_string(string, &USART::USART3);
}
