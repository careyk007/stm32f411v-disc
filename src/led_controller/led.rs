use embedded_hal::digital::v2::OutputPin;

enum LEDLevel {
    On,
    Off,
}

/// Implementation of an LED
/// 
/// # Purpose
/// 
/// This abstraction maintains a knowledge of its state, so
/// if you ask it to turn on while it is already on it will
/// not waste time setting the register.
/// 
/// I haven't done any testing to see if this is beneficial 
/// or not, but at least it was fun to implement!
/// 
/// # Example
/// 
/// ```
/// let p = stm32::Peripherals::take().unwrap();
/// let gpiod = p.GPIOD.split();
/// let red_led = gpiod.pd14.into_push_pull_output();
/// 
/// let led = LED::new(red_led);
/// 
/// led.on();
/// led.off();
/// led.off(); // Won't write to register since LED is already off
/// ```
pub struct LED<PIN>
where
    PIN: OutputPin,
{
    pin: PIN,
    state: LEDLevel,
}

impl<PIN> LED<PIN>
where
    PIN: OutputPin,
{
    // pub fn new(pin: PIN) -> Result<Self, ()> {
    pub fn new(pin: PIN) -> LED<PIN> {
        let mut led = LED { 
            pin, 
            state: LEDLevel::Off,
        };

        // Initialize the led however you want
        led.off();

        // Ok(led)
        led
    }

    pub fn on(&mut self) {
        match &self.state {
            LEDLevel::Off => {
                match self.pin.set_high() {
                    _ => {}
                }
                self.state = LEDLevel::On;
            },
            LEDLevel::On => {

            }
        }
    }

    pub fn off(&mut self) {
        match &self.state {
            LEDLevel::On => {
                match self.pin.set_low() {
                    _ => {}
                }
                self.state = LEDLevel::Off;
            },
            LEDLevel::Off => {

            }
        }
    }
}

impl<PIN> Drop for LED<PIN>
where 
    PIN: OutputPin,
{
    fn drop(&mut self) {
        self.off();
    }
}