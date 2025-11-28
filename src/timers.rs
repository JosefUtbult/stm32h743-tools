use crate::{
    interrupts::{disable_interrupt, enable_interrupt},
    register_tools::{clear_bit, read_register, set_bit, write_register},
    registers,
};

enum Timer {
    Tim2,
    Tim3,
    Tim4,
    Tim5,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TimerError {
    InvalidClockSpeed(u32),
    InvalidInterval(u16),
}

fn setup_cyclical_timer(
    timer: &Timer,
    clock_frequency: u32,
    interval_ms: u16,
) -> Result<(), TimerError> {
    use registers::{
        rcc::{APB1LENR, apb1lenr},
        tim2, tim3, tim4, tim5,
    };

    if clock_frequency == 0 {
        return Err(TimerError::InvalidClockSpeed(clock_frequency));
    }

    if interval_ms == 0 {
        return Err(TimerError::InvalidInterval(interval_ms));
    }

    // Trigger a clock tick every microsecond
    // 1 tick = 1 us
    let prescaler: u32 = (clock_frequency / 1_000_000) - 1;

    // Adjust the auto reload register to count up every microsecond to the desired interval in
    // milliseconds
    let auto_reload: u32 = (interval_ms as u32 * 1_000) - 1;

    unsafe {
        let apb1lenr_clock_field = match timer {
            Timer::Tim2 => apb1lenr::TIM2EN,
            Timer::Tim3 => apb1lenr::TIM3EN,
            Timer::Tim4 => apb1lenr::TIM4EN,
            Timer::Tim5 => apb1lenr::TIM5EN,
        };

        // Enable the clock for the specified timer
        set_bit(APB1LENR, apb1lenr_clock_field);

        let psc_prescaler_register = match timer {
            Timer::Tim2 => tim2::PSC,
            Timer::Tim3 => tim3::PSC,
            Timer::Tim4 => tim4::PSC,
            Timer::Tim5 => tim5::PSC,
        };

        // Write the pre-scaler into the pre-scaler register
        write_register(psc_prescaler_register, prescaler);

        let arr_auto_load_register = match timer {
            Timer::Tim2 => tim2::ARR,
            Timer::Tim3 => tim3::ARR,
            Timer::Tim4 => tim4::ARR,
            Timer::Tim5 => tim5::ARR,
        };

        // Write the auto reload register
        write_register(arr_auto_load_register, auto_reload);

        let egr_event_generator_register = get_egr_event_generator_register(timer);

        // Enable update generation in the event generator register
        set_bit(egr_event_generator_register, tim2::egr::UG);

        let dier_interrupt_register = get_dier_interrupt_register(timer);

        // Enable the update interrupt for the timer
        set_bit(dier_interrupt_register, tim2::dier::UIE);

        let nvic_interrupt_id = get_nvic_interrupt_id(timer);

        // Enable the timer interrupt in the NVIC
        enable_interrupt(nvic_interrupt_id);

        let cr1_control_register = get_cr1_control_register(timer);

        // Enable the timer
        set_bit(cr1_control_register, tim2::cr1::CEN);
    }

    Ok(())
}

fn cleanup_timer(timer: &Timer) {
    use registers::tim2;

    unsafe {
        let cr1_control_register = get_cr1_control_register(timer);

        // Disable the timer
        clear_bit(cr1_control_register, tim2::cr1::CEN);

        let dier_interrupt_register = get_dier_interrupt_register(timer);

        // Disable the update interrupt
        clear_bit(dier_interrupt_register, tim2::dier::UIE);

        let egr_event_generator_register = get_egr_event_generator_register(timer);

        // Trigger an update event to reload the registers
        set_bit(egr_event_generator_register, tim2::egr::UG);

        let nvic_interrupt_id = get_nvic_interrupt_id(timer);

        // Disable the nvic interrupt
        disable_interrupt(nvic_interrupt_id);
    }
}

fn get_now_us(timer: &Timer) -> u64 {
    use registers::{tim2, tim3, tim4, tim5};

    // Get the address of the CNT register for the selected timer
    let cnt_register = match timer {
        Timer::Tim2 => tim2::CNT,
        Timer::Tim3 => tim3::CNT,
        Timer::Tim4 => tim4::CNT,
        Timer::Tim5 => tim5::CNT,
    };

    // Read the current counter value (microseconds)
    let current_us = unsafe { read_register(cnt_register) as u64 };

    // Convert µs → ns
    current_us * 1_000
}

fn get_now_ns(timer: &Timer) -> u64 {
    get_now_us(timer) * 1_000
}

fn get_cr1_control_register(timer: &Timer) -> *mut u32 {
    use registers::{tim2, tim3, tim4, tim5};

    match timer {
        Timer::Tim2 => tim2::CR1,
        Timer::Tim3 => tim3::CR1,
        Timer::Tim4 => tim4::CR1,
        Timer::Tim5 => tim5::CR1,
    }
}

fn get_dier_interrupt_register(timer: &Timer) -> *mut u32 {
    use registers::{tim2, tim3, tim4, tim5};

    match timer {
        Timer::Tim2 => tim2::DIER,
        Timer::Tim3 => tim3::DIER,
        Timer::Tim4 => tim4::DIER,
        Timer::Tim5 => tim5::DIER,
    }
}

fn get_egr_event_generator_register(timer: &Timer) -> *mut u32 {
    use registers::{tim2, tim3, tim4, tim5};

    match timer {
        Timer::Tim2 => tim2::EGR,
        Timer::Tim3 => tim3::EGR,
        Timer::Tim4 => tim4::EGR,
        Timer::Tim5 => tim5::EGR,
    }
}

fn get_nvic_interrupt_id(timer: &Timer) -> u32 {
    use registers::irq;
    match timer {
        Timer::Tim2 => irq::TIM2_IRQ,
        Timer::Tim3 => irq::TIM3_IRQ,
        Timer::Tim4 => irq::TIM4_IRQ,
        Timer::Tim5 => irq::TIM5_IRQ,
    }
}

pub fn setup_cyclical_timer2(clock_frequency: u32, interval_ms: u16) -> Result<(), TimerError> {
    setup_cyclical_timer(&Timer::Tim2, clock_frequency, interval_ms)
}

pub fn setup_cyclical_timer3(clock_frequency: u32, interval_ms: u16) -> Result<(), TimerError> {
    setup_cyclical_timer(&Timer::Tim3, clock_frequency, interval_ms)
}

pub fn setup_cyclical_timer4(clock_frequency: u32, interval_ms: u16) -> Result<(), TimerError> {
    setup_cyclical_timer(&Timer::Tim4, clock_frequency, interval_ms)
}

pub fn setup_cyclical_timer5(clock_frequency: u32, interval_ms: u16) -> Result<(), TimerError> {
    setup_cyclical_timer(&Timer::Tim5, clock_frequency, interval_ms)
}

pub fn cleanup_timer2() {
    cleanup_timer(&Timer::Tim2);
}

pub fn cleanup_timer3() {
    cleanup_timer(&Timer::Tim3);
}

pub fn cleanup_timer4() {
    cleanup_timer(&Timer::Tim4);
}

pub fn cleanup_timer5() {
    cleanup_timer(&Timer::Tim5);
}

pub fn get_timer2_now_us() -> u64 {
    get_now_us(&Timer::Tim2)
}

pub fn get_timer3_now_us() -> u64 {
    get_now_us(&Timer::Tim3)
}

pub fn get_timer4_now_us() -> u64 {
    get_now_us(&Timer::Tim4)
}

pub fn get_timer5_now_us() -> u64 {
    get_now_us(&Timer::Tim5)
}

pub fn get_timer2_now_ns() -> u64 {
    get_now_ns(&Timer::Tim2)
}

pub fn get_timer3_now_ns() -> u64 {
    get_now_ns(&Timer::Tim3)
}

pub fn get_timer4_now_ns() -> u64 {
    get_now_ns(&Timer::Tim4)
}

pub fn get_timer5_now_ns() -> u64 {
    get_now_ns(&Timer::Tim5)
}

pub fn clear_timer2_interrupt_flag() {
    use registers::tim2::{SR, sr::UIF};
    unsafe {
        clear_bit(SR, UIF);
    }
}

pub fn clear_timer3_interrupt_flag() {
    use registers::tim3::{SR, sr::UIF};
    unsafe {
        clear_bit(SR, UIF);
    }
}

pub fn clear_timer4_interrupt_flag() {
    use registers::tim4::{SR, sr::UIF};
    unsafe {
        clear_bit(SR, UIF);
    }
}

pub fn clear_timer5_interrupt_flag() {
    use registers::tim5::{SR, sr::UIF};
    unsafe {
        clear_bit(SR, UIF);
    }
}
