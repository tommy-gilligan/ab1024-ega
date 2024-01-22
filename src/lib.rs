#![no_std]

pub mod color;
pub mod error;
mod registers;

#[cfg(feature = "graphics")]
mod graphics;
#[cfg(test)]
mod test;

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
    spi::SpiDevice,
};
use error::Error;

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
    buffer: [u8; (WIDTH * HEIGHT) / 2],
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

    fn reset_panel(&mut self) -> Result<(), Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.rst.set_low().map_err(Error::ResetPin)?;
        self.delay.delay_ms(1u32);
        self.rst.set_high().map_err(Error::ResetPin)?;
        self.delay.delay_ms(200u32);
        Ok(())
    }

    fn send_command(
        &mut self,
        command: u8,
    ) -> Result<(), Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.dc.set_low().map_err(Error::DataCommandPin)?;
        self.spi.write(&[command]).map_err(Error::Spi)?;
        Ok(())
    }

    fn send_data(
        &mut self,
        data: &[u8],
    ) -> Result<(), Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.dc.set_high().map_err(Error::DataCommandPin)?;
        self.spi.write(data).map_err(Error::Spi)?;
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.wakeup()?;
        self.sleep()?;
        Ok(())
    }

    fn sleep(&mut self) -> Result<(), Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.delay.delay_ms(10u32);
        self.send_command(registers::DEEP_SLEEP_REGISTER)?;
        self.send_data(&[0xA5])?;
        self.delay.delay_ms(100u32);
        self.rst.set_low().map_err(Error::ResetPin)?;
        self.dc.set_low().map_err(Error::DataCommandPin)?;
        Ok(())
    }

    fn wakeup(&mut self) -> Result<(), Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.reset_panel()?;

        while self.busy.is_low().map_err(Error::BusyPin)? {}

        self.send_command(registers::PANEL_SET_REGISTER)?;
        self.send_data(&[0xEF, 0x08])?;

        self.send_command(registers::POWER_SET_REGISTER)?;
        self.send_data(&[0x37, 0x00, 0x05, 0x05])?;

        self.send_command(registers::POWER_OFF_SEQ_SET_REGISTER)?;
        self.send_data(&[0x00])?;

        self.send_command(registers::BOOSTER_SOFTSTART_REGISTER)?;
        self.send_data(&[0xC7, 0xC7, 0x1D])?;

        self.send_command(registers::TEMP_SENSOR_EN_REGISTER)?;
        self.send_data(&[0x00])?;

        self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)?;
        self.send_data(&[0x37])?;

        self.send_command(0x60)?;
        self.send_data(&[0x20])?;

        self.send_command(registers::RESOLUTION_SET_REGISTER)?;
        self.send_data(&[0x02, 0x58, 0x01, 0xC0])?;

        self.send_command(0xE3)?;
        self.send_data(&[0xAA])?;

        self.delay.delay_ms(100u32);
        self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)?;
        self.send_data(&[0x37])?;
        Ok(())
    }

    pub fn display(&mut self) -> Result<(), Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.wakeup()?;

        self.send_command(registers::RESOLUTION_SET_REGISTER)?;
        self.send_data(&[0x02, 0x58, 0x01, 0xc0])?;

        self.send_command(registers::DATA_START_TRANS_REGISTER)?;
        self.dc.set_high().map_err(Error::DataCommandPin)?;

        self.spi.write(&self.buffer).map_err(Error::Spi)?;

        self.send_command(registers::POWER_OFF_REGISTER)?;
        while self.busy.is_low().map_err(Error::BusyPin)? {}

        self.send_command(registers::DISPLAY_REF_REGISTER)?;
        while self.busy.is_low().map_err(Error::BusyPin)? {}

        self.send_command(registers::POWER_OFF_REGISTER)?;
        while self.busy.is_high().map_err(Error::BusyPin)? {}

        self.delay.delay_ms(200u32);
        self.sleep()?;
        Ok(())
    }

    pub fn clear(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set_pixel(x, y, color::Color::WHITE);
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: color::Color) {
        assert!(x < WIDTH);
        assert!(y < HEIGHT);

        let index = (x >> 1) + y * WIDTH / 2;
        let color: u8 = color.into();

        if x % 2 == 0 {
            self.buffer[index] = (self.buffer[index] & 0x0f) | (color << 4);
        } else {
            self.buffer[index] = (self.buffer[index] & 0xf0) | color;
        }
    }
}
