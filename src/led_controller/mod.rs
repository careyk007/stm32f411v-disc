mod led;
use led::LED;
use embedded_hal::digital::v2::OutputPin;

enum LEDState {
    RED,
    BLUE,
    GREEN,
    ORANGE,
}

/// Implementation for an LED statemachine
/// 
/// # Purpose
/// 
/// This state machine turns one LED on at a time in a circular pattern.
/// The use case for this is to ensure that your code is running properly,
/// or simply for enjoyment.
/// 
/// # Example
/// 
/// ```
/// #![no_std]
/// #![no_main]
/// 
/// extern crate panic_halt;
/// 
/// use stm32f411_disco::led_controller::LEDController;
/// 
/// use cortex_m_rt::entry;
/// 
/// use stm32f4xx_hal::{
///     prelude::*;
///     stm32,
///     timer::Timer,
/// };
/// 
/// #[entry]
/// fn main() -> ! {
///     let cp = cortex_m::Peripherals::take().unwrap();
///     let p = stm32::Peripherals::take().unwrap();
///     let rcc = p.RCC.constrain();
///     let clocks = rcc.cfgr.freeze();
/// 
///     let gpiod = p.GPIOD.split();
///     let red_led = gpiod.pd14.into_push_pull_output();
///     let green_led = gpiod.pd12.into_push_pull_output();
///     let blue_led = gpiod.pd15.into_push_pull_output();
///     let orange_led = gpiod.pd13.into_push_pull_output();
/// 
///     let mut led_machine = LEDController::new(red_led, green_led, blue_led, orange_led);
/// 
///     let timer = Timer::syst(cp.SYST, 1.hz(), clocks);
/// 
///     loop {
///         nb::block!(timer.wait()).unwrap();
///         led_machine.advance_state();
///     }
/// }
/// ```
pub struct LEDController<RED, GREEN, BLUE, ORANGE>
where
    RED: OutputPin,
    GREEN: OutputPin,
    BLUE: OutputPin,
    ORANGE: OutputPin,
{
    red: LED<RED>,
    green: LED<GREEN>,
    blue: LED<BLUE>,
    orange: LED<ORANGE>,
    state: LEDState,
}

impl<RED, GREEN, BLUE, ORANGE> LEDController<RED, GREEN, BLUE, ORANGE>
where
    RED: OutputPin,
    GREEN: OutputPin,
    BLUE: OutputPin,
    ORANGE: OutputPin,
{
    pub fn new(red: RED, green: GREEN, blue: BLUE, orange: ORANGE) -> Self {
        let red = LED::new(red);
        let green = LED::new(green);
        let blue = LED::new(blue);
        let orange = LED::new(orange);

        let mut led_controller = LEDController {
            red,
            green,
            blue,
            orange,
            state: LEDState::BLUE,
        };

        // Inintialize?
        led_controller.reset_state();

        led_controller
    }

    pub fn reset_state(&mut self) {
        self.red.off();
        self.green.off();
        self.blue.off();
        self.orange.off();
        self.state = LEDState::BLUE;
    }

    pub fn advance_state(&mut self) {
        match &self.state {
            LEDState::BLUE => {
                self.red.off();
                self.blue.on();
                self.green.off();
                self.orange.off();
                self.state = LEDState::GREEN;
            },
            LEDState::GREEN => {
                self.red.off();
                self.blue.off();
                self.green.on();
                self.orange.off();
                self.state = LEDState::ORANGE;
            },
            LEDState::ORANGE => {
                self.red.off();
                self.blue.off();
                self.green.off();
                self.orange.on();
                self.state = LEDState::RED;
            },
            LEDState::RED => {
                self.red.on();
                self.blue.off();
                self.green.off();
                self.orange.off();
                self.state = LEDState::BLUE;
            },
        }
    }
}
