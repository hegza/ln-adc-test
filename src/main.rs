#![no_std]
#![no_main]

use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{OriginDimensions, Point, Primitive, RgbColor, Size};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use longan_nano::hal::adc::Adc;
use longan_nano::hal::delay::McycleDelay;
use longan_nano::hal::prelude::*;
use panic_halt as _;

use embedded_graphics::mono_font::ascii::FONT_5X8;
use longan_nano::hal::{pac, rcu::RcuExt};
use longan_nano::{lcd, lcd_pins};
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    // Constrain PAC I/O pins/interfaces for use in the LCD driver
    let mut afio = dp.AFIO.constrain(&mut rcu);
    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    // Clear screen
    Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(&mut lcd)
        .unwrap();

    let style = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(Rgb565::BLACK)
        .background_color(Rgb565::GREEN)
        .build();

    let mut delay = McycleDelay::new(&rcu.clocks);

    // let's try the ADC
    let mut adc = Adc::adc0(dp.ADC0, &mut rcu);
    let mut adc_pin = gpioa.pa0.into_analog();

    loop {
        // Read ADC
        let reading: u32 = adc.read(&mut adc_pin).unwrap();

        // Create a text at position (20, 30) and draw it using style defined above
        let mut text = heapless::String::<32>::new();
        text.push_str("ADC output: ")
            .expect("failed to make string");
        text.push_str(&heapless::String::<32>::from(reading))
            .expect("failed to make string");

        Text::new(text.as_str(), Point::new(40, 35), style)
            .draw(&mut lcd)
            .unwrap();

        delay.delay_ms(500);
    }
}
