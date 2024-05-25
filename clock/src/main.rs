#![no_std]
#![no_main]
#![allow(unused_imports)] // Add this line to allow unused imports

use core::cell::RefCell;
use core::panic::PanicInfo;
use embassy_executor::Spawner;

// GPIO
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::{PIN_0, PIN_12, PIN_13, PIN_14, PIN_15, PIN_16, PIN_17, SPI0};

// PWM
use embassy_rp::pwm::{Config as PwmConfig, Pwm};

// ADC
use embassy_rp::adc::{
    Adc, Async, Channel as AdcChannel, Config as AdcConfig, InterruptHandler as InterruptHandlerAdc,
};

// USB
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};
use log::info;

// Channel
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};

// Timer
use embassy_time::{Delay, Timer};

// Select futures
use embassy_futures::select::select;
use embassy_futures::select::Either::{First, Second};

// Display
use core::fmt::Write;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::spi;
use embassy_rp::spi::{Blocking, Spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::renderer::CharacterStyle;
use embedded_graphics::text::Text;
use heapless::String;
use lab05_ex3_4_5::SPIDeviceInterface;
use st7789::{Orientation, ST7789};

const DISPLAY_FREQ: u32 = 64_000_000;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    ADC_IRQ_FIFO => InterruptHandlerAdc;
});


#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // The USB driver, for serial debugging, you might need it ;)
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // ------------------------ DISPLAY ----------------------------

    // FONT STYLE
    let mut style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);
    style.set_background_color(Some(Rgb565::BLACK));

    // ************** Display initialization - DO NOT MODIFY! *****************
    let miso = peripherals.PIN_4;
    let display_cs = peripherals.PIN_17;
    let mosi = peripherals.PIN_19;
    let clk = peripherals.PIN_18;
    let rst = peripherals.PIN_0;
    let dc = peripherals.PIN_16;
    let mut display_config = spi::Config::default();
    display_config.frequency = DISPLAY_FREQ;
    display_config.phase = spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = spi::Polarity::IdleHigh;

    // Init SPI
    let spi: Spi<'_, _, Blocking> =
        Spi::new_blocking(peripherals.SPI0, clk, mosi, miso, display_config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

    let display_spi = SpiDeviceWithConfig::new(
        &spi_bus,
        Output::new(display_cs, Level::High),
        display_config,
    );

    let dc = Output::new(dc, Level::Low);
    let rst = Output::new(rst, Level::Low);
    let di = SPIDeviceInterface::new(display_spi, dc);

    // Init ST7789 LCD
    let mut display = ST7789::new(di, rst, 240, 240);
    display.init(&mut Delay).unwrap();
    display.set_orientation(Orientation::Portrait).unwrap();
    display.clear(Rgb565::BLACK).unwrap();
    // ************************************************************************

    // Clear display
    display.clear(Rgb565::BLACK).unwrap();

    let mut text = String::<64>::new();
    write!(text, "Info").unwrap(); // led_color must be defined first

    Text::new(&text, Point::new(40, 110), style)
        .draw(&mut display)
        .unwrap();

    // Small delay for yielding
    Timer::after_millis(1).await;
}