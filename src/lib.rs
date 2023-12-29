#![no_std]

pub mod color;
#[cfg(feature = "graphics")]
mod graphics;
mod registers;

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
    spi::SpiDevice,
};

pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 448;

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
    pub buffer: [u8; (WIDTH * HEIGHT) / 2],
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
            buffer: [0b00010001; (WIDTH * HEIGHT) / 2],
        }
    }

    fn reset_panel(&mut self) -> Result<(), <RST as embedded_hal::digital::ErrorType>::Error> {
        self.rst.set_low()?;
        self.delay.delay_ms(1u32);
        self.rst.set_high()?;
        self.delay.delay_ms(200u32);
        Ok(())
    }

    fn send_command(
        &mut self,
        command: u8,
    ) -> Result<(), <DC as embedded_hal::digital::ErrorType>::Error> {
        self.dc.set_low()?;
        self.spi.write(&[command]).unwrap();
        Ok(())
    }

    fn send_data(
        &mut self,
        data: &[u8],
    ) -> Result<(), <DC as embedded_hal::digital::ErrorType>::Error> {
        self.dc.set_high()?;
        self.spi.write(data).unwrap();
        Ok(())
    }

    pub fn begin(&mut self) -> bool {
        match self.set_panel_deep_sleep(false) {
            Ok(true) => {
                self.set_panel_deep_sleep(true).unwrap();
                true
            }
            _ => false,
        }
    }

    fn set_panel_deep_sleep(
        &mut self,
        state: bool,
    ) -> Result<bool, <BUSY as embedded_hal::digital::ErrorType>::Error> {
        if state {
            self.delay.delay_ms(10u32);
            self.send_command(registers::DEEP_SLEEP_REGISTER).unwrap();
            self.send_data(&[0xA5]).unwrap();
            self.delay.delay_ms(100u32);
            self.rst.set_low().unwrap();
            self.dc.set_low().unwrap();
            Ok(true)
        } else {
            self.reset_panel().unwrap();

            while self.busy.is_low()? {}
            if self.busy.is_low()? {
                return Ok(false);
            }

            let panel_set_data: [u8; 2] = [0xEF, 0x08];
            self.send_command(registers::PANEL_SET_REGISTER).unwrap();
            self.send_data(&panel_set_data).unwrap();

            let power_set_data: [u8; 4] = [0x37, 0x00, 0x05, 0x05];
            self.send_command(registers::POWER_SET_REGISTER).unwrap();
            self.send_data(&power_set_data).unwrap();

            self.send_command(registers::POWER_OFF_SEQ_SET_REGISTER)
                .unwrap();
            self.send_data(&[registers::PANEL_SET_REGISTER]).unwrap();

            let booster_softstart_data: [u8; 3] = [0xC7, 0xC7, 0x1D];
            self.send_command(registers::BOOSTER_SOFTSTART_REGISTER)
                .unwrap();
            self.send_data(&booster_softstart_data).unwrap();

            self.send_command(registers::TEMP_SENSOR_EN_REGISTER)
                .unwrap();
            self.send_data(&[registers::PANEL_SET_REGISTER]).unwrap();

            self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)
                .unwrap();
            self.send_data(&[0x37]).unwrap();

            self.send_command(0x60).unwrap();
            self.send_data(&[0x20]).unwrap();

            let res_set_data: [u8; 4] = [0x02, 0x58, 0x01, 0xC0];
            self.send_command(registers::RESOLUTION_SET_REGISTER)
                .unwrap();
            self.send_data(&res_set_data).unwrap();

            self.send_command(0xE3).unwrap();
            self.send_data(&[0xAA]).unwrap();

            self.delay.delay_ms(100u32);
            self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)
                .unwrap();
            self.send_data(&[0x37]).unwrap();
            Ok(true)
        }
    }

    pub fn display(&mut self) -> Result<(), <BUSY as embedded_hal::digital::ErrorType>::Error> {
        self.set_panel_deep_sleep(false).unwrap();

        let res_set_data: [u8; 4] = [0x02, 0x58, 0x01, 0xc0];
        self.send_command(registers::RESOLUTION_SET_REGISTER)
            .unwrap();
        self.send_data(&res_set_data).unwrap();

        self.send_command(registers::DATA_START_TRANS_REGISTER)
            .unwrap();
        self.dc.set_high().unwrap();

        self.spi.write(&self.buffer).unwrap();

        self.send_command(registers::POWER_OFF_REGISTER).unwrap();
        while self.busy.is_low()? {}

        self.send_command(registers::DISPLAY_REF_REGISTER).unwrap();
        while self.busy.is_low()? {}

        self.send_command(registers::POWER_OFF_REGISTER).unwrap();
        while self.busy.is_high()? {}

        self.delay.delay_ms(200u32);
        self.set_panel_deep_sleep(true).unwrap();
        Ok(())
    }
}
