#![no_std]

mod color;
mod registers;
#[cfg(feature="dep:embedded-graphics-core")]
mod graphics;

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
    spi::SpiDevice,
};

pub struct Epd<D, S, RST, DC, BUSY>
where
    D: DelayNs,
    S: SpiDevice,
    RST: OutputPin,
    DC: OutputPin,
    BUSY: InputPin,
{
    spi: S,
    rst: RST,
    dc: DC,
    busy: BUSY,
    delay: D,
    buffer: [u8; (600 * 448) / 2],
}

impl<D, S, RST, DC, BUSY> Epd<D, S, RST, DC, BUSY>
where
    D: DelayNs,
    S: SpiDevice,
    RST: OutputPin,
    DC: OutputPin,
    BUSY: InputPin,
{
    pub fn new(spi: S, rst: RST, dc: DC, busy: BUSY, delay: D) -> Self {
        Self {
            spi,
            rst,
            dc,
            busy,
            delay,
            buffer: [0b00100010; (600 * 448) / 2],
        }
    }

    fn reset_panel(&mut self) {
        self.rst.set_low().unwrap();
        self.delay.delay_ms(1u32);
        self.rst.set_high().unwrap();
        self.delay.delay_ms(200u32);
    }

    fn send_command(&mut self, command: u8) {
        self.dc.set_low().unwrap();
        self.spi.write(&[command]).unwrap();
    }

    fn send_data(&mut self, data: &[u8]) {
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

    fn set_panel_deep_sleep(&mut self, state: bool) -> bool {
        if state {
            self.delay.delay_ms(10u32);
            self.send_command(registers::DEEP_SLEEP_REGISTER);
            self.send_data(&[0xA5]);
            self.delay.delay_ms(100u32);
            self.rst.set_low().unwrap();
            self.dc.set_low().unwrap();
            true
        } else {
            self.reset_panel();

            while self.busy.is_low().unwrap() {}
            if self.busy.is_low().unwrap() {
                return false;
            }

            let panel_set_data: [u8; 2] = [0xEF, 0x08];
            self.send_command(registers::PANEL_SET_REGISTER);
            self.send_data(&panel_set_data);

            let power_set_data: [u8; 4] = [0x37, 0x00, 0x05, 0x05];
            self.send_command(registers::POWER_SET_REGISTER);
            self.send_data(&power_set_data);

            self.send_command(registers::POWER_OFF_SEQ_SET_REGISTER);
            self.send_data(&[registers::PANEL_SET_REGISTER]);

            let booster_softstart_data: [u8; 3] = [0xC7, 0xC7, 0x1D];
            self.send_command(registers::BOOSTER_SOFTSTART_REGISTER);
            self.send_data(&booster_softstart_data);

            self.send_command(registers::TEMP_SENSOR_EN_REGISTER);
            self.send_data(&[registers::PANEL_SET_REGISTER]);

            self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER);
            self.send_data(&[0x37]);

            self.send_command(0x60);
            self.send_data(&[0x20]);

            let res_set_data: [u8; 4] = [0x02, 0x58, 0x01, 0xC0];
            self.send_command(registers::RESOLUTION_SET_REGISTER);
            self.send_data(&res_set_data);

            self.send_command(0xE3);
            self.send_data(&[0xAA]);

            self.delay.delay_ms(100u32);
            self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER);
            self.send_data(&[0x37]);
            true
        }
    }

    pub fn display(&mut self) {
        self.set_panel_deep_sleep(false);

        let res_set_data: [u8; 4] = [0x02, 0x58, 0x01, 0xc0];
        self.send_command(registers::RESOLUTION_SET_REGISTER);
        self.send_data(&res_set_data);

        self.send_command(registers::DATA_START_TRANS_REGISTER);
        self.dc.set_high().unwrap();

        self.spi.write(&self.buffer).unwrap();

        self.send_command(registers::POWER_OFF_REGISTER);
        while self.busy.is_low().unwrap() {}

        self.send_command(registers::DISPLAY_REF_REGISTER);
        while self.busy.is_low().unwrap() {}

        self.send_command(registers::POWER_OFF_REGISTER);
        while self.busy.is_high().unwrap() {}

        self.delay.delay_ms(200u32);
        self.set_panel_deep_sleep(true);
    }
}
