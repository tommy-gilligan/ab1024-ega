#![no_std]
#![doc = ::embed_doc_image::embed_image!("image-photo", "examples/image_photo.jpg")]
#![doc = include_str!("../README.md")]

pub mod color;
pub mod error;
mod registers;
mod util;

#[cfg(feature = "graphics")]
mod graphics;
#[cfg(test)]
mod test;

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
    spi::SpiDevice,
};

use util::Sealed;
pub trait State: Sealed {}
pub enum Uninitialized {}
pub enum Initialized {}
impl State for Uninitialized {}
impl State for Initialized {}
impl Sealed for Uninitialized {}
impl Sealed for Initialized {}

pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 448;

pub struct Display<D, S, RST, DC, BUSY, St: State>
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
    state: core::marker::PhantomData<St>,
}

impl<D, S, RST, DC, BUSY> Display<D, S, RST, DC, BUSY, Uninitialized>
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
            buffer: [0b0001_0001; (WIDTH * HEIGHT) / 2],
            state: core::marker::PhantomData,
        }
    }

    /// # Errors
    pub fn init(
        mut self,
    ) -> Result<
        Display<D, S, RST, DC, BUSY, Initialized>,
        error::Error<BUSY::Error, RST::Error, DC::Error, S::Error>,
    > {
        self.wakeup()?;
        self.sleep()?;

        Ok(Display {
            spi: self.spi,
            rst: self.rst,
            dc: self.dc,
            busy: self.busy,
            delay: self.delay,
            buffer: self.buffer,
            state: core::marker::PhantomData,
        })
    }
}

impl<D, S, RST, DC, BUSY> Display<D, S, RST, DC, BUSY, Initialized>
where
    D: DelayNs,
    S: SpiDevice,
    RST: OutputPin,
    DC: OutputPin,
    BUSY: InputPin,
{
    /// # Errors
    pub fn display(
        &mut self,
    ) -> Result<(), error::Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.wakeup()?;

        self.send_command(registers::RESOLUTION_SET_REGISTER)?;
        self.send_data(&[0x02, 0x58, 0x01, 0xc0])?;

        self.send_command(registers::DATA_START_TRANS_REGISTER)?;
        self.dc.set_high().map_err(error::Error::DataCommandPin)?;

        self.spi.write(&self.buffer).map_err(error::Error::Spi)?;

        self.send_command(registers::POWER_OFF_REGISTER)?;
        while self.busy.is_low().map_err(error::Error::BusyPin)? {}

        self.send_command(registers::DISPLAY_REF_REGISTER)?;
        while self.busy.is_low().map_err(error::Error::BusyPin)? {}

        self.send_command(registers::POWER_OFF_REGISTER)?;
        while self.busy.is_high().map_err(error::Error::BusyPin)? {}

        self.delay.delay_ms(200u32);
        self.sleep()
    }
}

impl<D, S, RST, DC, BUSY, St: State> Display<D, S, RST, DC, BUSY, St>
where
    D: DelayNs,
    S: SpiDevice,
    RST: OutputPin,
    DC: OutputPin,
    BUSY: InputPin,
{
    /// # Errors
    pub fn clear(
        &mut self,
    ) -> Result<(), error::Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set_pixel(x, y, color::Color::WHITE)?;
            }
        }
        Ok(())
    }

    /// # Errors
    pub fn set_pixel(
        &mut self,
        x: usize,
        y: usize,
        color: color::Color,
    ) -> Result<(), error::Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        let color: u8 = color.into();

        if let Some(byte) = self.buffer.get_mut((x >> 1) + y * WIDTH / 2) {
            if x % 2 == 0 {
                *byte = (*byte & 0x0f) | (color << 4);
            } else {
                *byte = (*byte & 0xf0) | color;
            }
            Ok(())
        } else {
            Err(error::Error::PixelOutOfBounds)
        }
    }

    fn reset_panel(
        &mut self,
    ) -> Result<(), error::Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.rst.set_low().map_err(error::Error::ResetPin)?;
        self.delay.delay_ms(1u32);
        self.rst.set_high().map_err(error::Error::ResetPin)?;
        self.delay.delay_ms(200u32);
        Ok(())
    }

    fn send_command(
        &mut self,
        command: u8,
    ) -> Result<(), error::Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.dc.set_low().map_err(error::Error::DataCommandPin)?;
        self.spi.write(&[command]).map_err(error::Error::Spi)
    }

    fn send_data(
        &mut self,
        data: &[u8],
    ) -> Result<(), error::Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.dc.set_high().map_err(error::Error::DataCommandPin)?;
        self.spi.write(data).map_err(error::Error::Spi)
    }

    fn sleep(&mut self) -> Result<(), error::Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.delay.delay_ms(10u32);
        self.send_command(registers::DEEP_SLEEP_REGISTER)?;
        self.send_data(&[0xA5])?;
        self.delay.delay_ms(100u32);
        self.rst.set_low().map_err(error::Error::ResetPin)?;
        self.dc.set_low().map_err(error::Error::DataCommandPin)
    }

    fn wakeup(&mut self) -> Result<(), error::Error<BUSY::Error, RST::Error, DC::Error, S::Error>> {
        self.reset_panel()?;

        while self.busy.is_low().map_err(error::Error::BusyPin)? {}

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
        self.send_data(&[0x37])
    }
}
