#![no_std]

pub mod color;
mod registers;

#[cfg(feature = "graphics")]
mod graphics;

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
    spi::SpiDevice,
};

#[derive(Debug)]
pub enum Error<PS>
where
    PS: embedded_hal::digital::Error,
{
    PinSet(PS),
    Spi,
}

impl<T> From<T> for Error<T>
where
    T: embedded_hal::digital::Error,
{
    fn from(e: T) -> Self {
        Self::PinSet(e)
    }
}

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

    fn reset_panel(&mut self) -> Result<(), Error<RST::Error>> {
        self.rst.set_low()?;
        self.delay.delay_ms(1u32);
        self.rst.set_high()?;
        self.delay.delay_ms(200u32);
        Ok(())
    }

    fn send_command(&mut self, command: u8) -> Result<(), Error<RST::Error>> {
        self.dc.set_low().unwrap();
        self.spi.write(&[command]).map_err(|_| Error::Spi)?;
        Ok(())
    }

    fn send_data(&mut self, data: &[u8]) -> Result<(), Error<RST::Error>> {
        self.dc.set_high().unwrap();
        self.spi.write(data).map_err(|_| Error::Spi)?;
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), Error<RST::Error>> {
        self.wakeup()?;
        self.sleep()?;
        Ok(())
    }

    fn sleep(&mut self) -> Result<(), Error<RST::Error>> {
        self.delay.delay_ms(10u32);
        self.send_command(registers::DEEP_SLEEP_REGISTER)?;
        self.send_data(&[0xA5])?;
        self.delay.delay_ms(100u32);
        self.rst.set_low()?;
        self.dc.set_low().unwrap();
        Ok(())
    }

    fn wakeup(&mut self) -> Result<(), Error<RST::Error>> {
        self.reset_panel()?;

        while self.busy.is_low().unwrap() {}

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

    pub fn display(&mut self) -> Result<(), Error<RST::Error>> {
        self.wakeup()?;

        self.send_command(registers::RESOLUTION_SET_REGISTER)?;
        self.send_data(&[0x02, 0x58, 0x01, 0xc0])?;

        self.send_command(registers::DATA_START_TRANS_REGISTER)?;
        self.dc.set_high().unwrap();

        self.spi.write(&self.buffer).map_err(|_| Error::Spi)?;

        self.send_command(registers::POWER_OFF_REGISTER)?;
        while self.busy.is_low().unwrap() {}

        self.send_command(registers::DISPLAY_REF_REGISTER)?;
        while self.busy.is_low().unwrap() {}

        self.send_command(registers::POWER_OFF_REGISTER)?;
        while self.busy.is_high().unwrap() {}

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

#[cfg(test)]
mod test {
    extern crate std;
    use super::*;
    use embedded_hal_mock::eh1::{
        pin::{Mock as PinMock, State as PinState, Transaction as PinTransaction},
        spi::{Mock as SpiMock, Transaction as SpiTransaction},
        top_level::{Expectation, Hal},
    };

    #[test]
    fn test_reset_panel() {
        let mut hal = Hal::new(&[
            Expectation::Digital(0, PinTransaction::set(PinState::Low)),
            Expectation::Delay(1_000_000),
            Expectation::Digital(0, PinTransaction::set(PinState::High)),
            Expectation::Delay(200_000_000),
        ]);

        let mut spi = SpiMock::new(&[]);
        let mut dc = PinMock::new(&[]);
        let mut busy = PinMock::new(&[]);

        let mut epd = Epd::new(
            spi.clone(),
            hal.clone().pin(0),
            dc.clone(),
            busy.clone(),
            hal.clone().delay(),
        );

        epd.reset_panel().unwrap();

        spi.done();
        dc.done();
        busy.done();
        hal.done();
    }

    #[test]
    fn test_send_command() {
        let dc = 0;
        let rst = 1;
        let busy = 2;

        let mut hal = Hal::new(&[
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(23)),
            Expectation::Spi(SpiTransaction::transaction_end()),
        ]);

        let mut epd = Epd::new(
            hal.clone().spi(),
            hal.clone().pin(rst),
            hal.clone().pin(dc),
            hal.clone().pin(busy),
            hal.clone().delay(),
        );

        epd.send_command(23).unwrap();

        hal.done();
    }

    #[test]
    fn test_send_data() {
        let dc = 0;
        let rst = 1;
        let busy = 2;

        let mut hal = Hal::new(&[
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([20, 45].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
        ]);

        let mut epd = Epd::new(
            hal.clone().spi(),
            hal.clone().pin(rst),
            hal.clone().pin(dc),
            hal.clone().pin(busy),
            hal.clone().delay(),
        );

        epd.send_data(&[20, 45]).unwrap();

        hal.done();
    }

    #[test]
    fn test_sleep_panel() {
        let dc = 0;
        let rst = 1;
        let busy = 2;

        let mut hal = Hal::new(&[
            Expectation::Delay(10_000_000),
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0x07)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0xA5)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Delay(100_000_000),
            Expectation::Digital(rst, PinTransaction::set(PinState::Low)),
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
        ]);

        Epd::new(
            hal.clone().spi(),
            hal.clone().pin(rst),
            hal.clone().pin(dc),
            hal.clone().pin(busy),
            hal.clone().delay(),
        )
        .sleep()
        .unwrap();

        hal.done();
    }

    #[test]
    fn test_wakeup() {
        let dc = 0;
        let rst = 1;
        let busy = 2;

        let mut hal = Hal::new(&[
            Expectation::Digital(rst, PinTransaction::set(PinState::Low)),
            Expectation::Delay(1_000_000),
            Expectation::Digital(rst, PinTransaction::set(PinState::High)),
            Expectation::Delay(200_000_000),
            // busy
            // busy
            // no longer busy
            Expectation::Digital(busy, PinTransaction::get(PinState::Low)),
            Expectation::Digital(busy, PinTransaction::get(PinState::Low)),
            Expectation::Digital(busy, PinTransaction::get(PinState::High)),
            // self.send_command(registers::PANEL_SET_REGISTER)?;
            // self.send_data(&[0xEF, 0x08])?;
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0x00)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([0xEF, 0x08].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
            // self.send_command(registers::POWER_SET_REGISTER)?;
            // self.send_data(&[0x37, 0x00, 0x05, 0x05])?;
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0x01)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([0x37, 0x00, 0x05, 0x05].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
            // self.send_command(registers::POWER_OFF_SEQ_SET_REGISTER)?;
            // self.send_data(&[0x00])?;
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0x03)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([0x00].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
            // self.send_command(registers::BOOSTER_SOFTSTART_REGISTER)?;
            // self.send_data(&[0xC7, 0xC7, 0x1D])?;
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0x06)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([0xC7, 0xC7, 0x1D].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
            // self.send_command(registers::TEMP_SENSOR_EN_REGISTER)?;
            // self.send_data(&[0x00])?;
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0x41)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([0x00].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
            // self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)?;
            // self.send_data(&[0x37])?;
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0x50)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([0x37].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
            // self.send_command(0x60)?;
            // self.send_data(&[0x20])?;
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0x60)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([0x20].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
            // self.send_command(registers::RESOLUTION_SET_REGISTER)?;
            // self.send_data(&[0x02, 0x58, 0x01, 0xC0])?;
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0x61)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([0x02, 0x58, 0x01, 0xC0].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
            // self.send_command(0xE3)?;
            // self.send_data(&[0xAA])?;
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0xE3)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([0xAA].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
            // self.delay.delay_ms(100u32);
            // self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)?;
            // self.send_data(&[0x37])?;
            Expectation::Delay(100_000_000),
            Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write(0x50)),
            Expectation::Spi(SpiTransaction::transaction_end()),
            Expectation::Digital(dc, PinTransaction::set(PinState::High)),
            Expectation::Spi(SpiTransaction::transaction_start()),
            Expectation::Spi(SpiTransaction::write_vec([0x37].to_vec())),
            Expectation::Spi(SpiTransaction::transaction_end()),
        ]);

        Epd::new(
            hal.clone().spi(),
            hal.clone().pin(rst),
            hal.clone().pin(dc),
            hal.clone().pin(busy),
            hal.clone().delay(),
        )
        .wakeup()
        .unwrap();

        hal.done();
    }
}
