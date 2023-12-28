use embedded_graphics_core::{
    prelude::{Point, OriginDimensions, Dimensions, Size, PixelColor},
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    primitives::Rectangle,
    Pixel
};
use embedded_graphics::pixelcolor::RgbColor;
use hal::{
    gpio::{self, Input, PushPull, Floating, GpioPin, Output},
    peripherals,
    spi::{master::Spi, FullDuplexMode},
    prelude::*,
    delay::Delay
};

pub struct Epd<'a> {
    spi: Spi<'a, peripherals::SPI2, FullDuplexMode>,
    rst: GpioPin<Output<PushPull>, 19>,
    dc: GpioPin<Output<PushPull>, 33>,
    busy: GpioPin<Input<Floating>, 32>,
    delay: Delay,
    buffer: [u8; (600 * 448) / 2]
}

impl <'a> Epd<'a> {
    pub fn new(
        spi: Spi<'a, peripherals::SPI2, FullDuplexMode>,
        rst: GpioPin<Output<PushPull>, 19>,
        dc: GpioPin<Output<PushPull>, 33>,
        busy: GpioPin<Input<Floating>, 32>,
        delay: Delay,
    ) -> Self {
        Self {
            spi,
            rst,
            dc,
            busy,
            delay,
            buffer: [0b00100010; (600 * 448) / 2]
        }
    }

    pub fn reset_panel(&mut self) {
        self.rst.set_low().unwrap();
        self.delay.delay_ms(1u32);
        self.rst.set_high().unwrap();
        self.delay.delay_ms(200u32);
    }

    pub fn send_command(&mut self, command: u8) {
        self.dc.set_low().unwrap();
        self.spi.write(&[command]).unwrap();
    }

    pub fn send_data(&mut self, data: &[u8]) {
        self.dc.set_high().unwrap();
        self.spi.write(data).unwrap();
    }

    pub fn begin(&mut self) -> bool {
        if self.set_panel_deep_sleep(false) {
            self.set_panel_deep_sleep(true);
            true
        } else {
            false
        }
    }

    pub fn set_panel_deep_sleep(&mut self, state: bool) -> bool {
        if state {
            self.delay.delay_ms(10u32);
            self.send_command(DEEP_SLEEP_REGISTER);
            self.send_data(&[0xA5]);
            self.delay.delay_ms(100u32);
            self.rst.set_low().unwrap();
            self.dc.set_low().unwrap();
            true
        } else {
            self.reset_panel();

            while self.busy.is_low().unwrap() {
            }
            if self.busy.is_low().unwrap() {
                return false;
            }

            // Send whole bunch of commands and data
            let panel_set_data: [u8; 2] = [0xEF, 0x08];
            self.send_command(PANEL_SET_REGISTER);
            self.send_data(&panel_set_data);

            let power_set_data: [u8; 4] = [0x37, 0x00, 0x05, 0x05];
            self.send_command(POWER_SET_REGISTER);
            self.send_data(&power_set_data);

            self.send_command(POWER_OFF_SEQ_SET_REGISTER);
            self.send_data(&[PANEL_SET_REGISTER]);

            let booster_softstart_data: [u8; 3] = [0xC7, 0xC7, 0x1D];
            self.send_command(BOOSTER_SOFTSTART_REGISTER);
            self.send_data(&booster_softstart_data);

            self.send_command(TEMP_SENSOR_EN_REGISTER);
            self.send_data(&[PANEL_SET_REGISTER]);

            self.send_command(VCOM_DATA_INTERVAL_REGISTER);
            self.send_data(&[0x37]);

            self.send_command(0x60);
            self.send_data(&[0x20]);

            let res_set_data: [u8; 4] = [0x02, 0x58, 0x01, 0xC0];
            self.send_command(RESOLUTION_SET_REGISTER);
            self.send_data(&res_set_data);

            self.send_command(0xE3);
            self.send_data(&[0xAA]);

            self.delay.delay_ms(100u32);
            self.send_command(VCOM_DATA_INTERVAL_REGISTER);
            self.send_data(&[0x37]);
            true
        }
    }

    pub fn display(&mut self) {
        self.set_panel_deep_sleep(false);

        let res_set_data: [u8; 4] = [0x02, 0x58, 0x01, 0xc0];
        self.send_command(RESOLUTION_SET_REGISTER);
        self.send_data(&res_set_data);

        self.send_command(DATA_START_TRANS_REGISTER);
        self.dc.set_high().unwrap();

        self.spi.write(&self.buffer).unwrap();

        self.send_command(POWER_OFF_REGISTER);
        while self.busy.is_low().unwrap() {
        }

        self.send_command(DISPLAY_REF_REGISTER);
        while self.busy.is_low().unwrap() {
        }

        self.send_command(POWER_OFF_REGISTER);
        while self.busy.is_high().unwrap() {
        }

        self.delay.delay_ms(200u32);
        self.set_panel_deep_sleep(true);
    }
}

impl OriginDimensions for Epd<'_> {
    fn size(&self) -> Size { 
        Size::new(600, 448)
    }
}

impl DrawTarget for Epd<'_> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error> where I: IntoIterator<Item = Pixel<Self::Color>> {
        for pixel in pixels {
            let point = pixel.0;
            let [r, g, b] = [pixel.1.r(), pixel.1.g(), pixel.1.b()];

            let nibble = match (r, g, b) {
                (0x00, 0x00, 0x00) => 0b00000000,
                (0xff, 0xff, 0xff) => 0b00000001,
                (0x00, 0xff, 0x00) => 0b00000010,
                (0x00, 160, 0x00) => 0b00000010,
                (0x00, 0x00, 0xff) => 0b00000011,
                (0xff, 0x00, 0x00) => 0b00000100,
                (0xff, 0xff, 0x00) => 0b00000101,
                _ => 0b00000110,
            };

            let index = (300usize * point.y as usize + (point.x as usize >> 1)).min(134399);

            if point.x % 2 == 0 {
                self.buffer[index] = (self.buffer[index] & 0x0f) | (nibble << 4);
            } else {
                self.buffer[index] = (self.buffer[index] & 0xf0) | nibble;
            }
        }

        Ok(())
   }
}

const BLACK: u8  = 0b00000000;
const WHITE: u8  = 0b00000001;
const GREEN: u8  = 0b00000010;
const BLUE: u8   = 0b00000011;
const RED: u8    = 0b00000100;
const YELLOW: u8 = 0b00000101;
const ORANGE: u8 = 0b00000110;

const PANEL_SET_REGISTER: u8 =          0x00;
const POWER_SET_REGISTER: u8 =          0x01;
const VCM_DC_SET_REGISTER: u8 =         0x02;
const POWER_OFF_SEQ_SET_REGISTER: u8 =  0x03;
const POWER_OFF_REGISTER: u8 =          0x04;
const BOOSTER_SOFTSTART_REGISTER: u8 =  0x06;
const DEEP_SLEEP_REGISTER: u8 =         0x07;

const DATA_START_TRANS_REGISTER: u8 =   0x10;
const DATA_STOP_REGISTER: u8 =          0x11;
const DISPLAY_REF_REGISTER: u8 =        0x12;
const IMAGE_PROCESS_REGISTER: u8 =      0x13;

const PLL_CONTROL_REGISTER: u8 =        0x30;

const TEMP_SENSOR_REGISTER: u8 =        0x40;
const TEMP_SENSOR_EN_REGISTER: u8 =     0x41;
const TEMP_SENSOR_WR_REGISTER: u8 =     0x42;
const TEMP_SENSOR_RD_REGISTER: u8 =     0x43;

const VCOM_DATA_INTERVAL_REGISTER: u8 = 0x50;
const LOW_POWER_DETECT_REGISTER: u8 =   0x51;
const RESOLUTION_SET_REGISTER: u8 =     0x61;
const STATUS_REGISTER: u8 =             0x71;
const VCOM_VALUE_REGISTER: u8 =         0x81;
