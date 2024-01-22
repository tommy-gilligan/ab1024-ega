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

    let mut epd = Display::new(
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

    let mut epd = Display::new(
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

    let mut epd = Display::new(
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

    Display::new(
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

    Display::new(
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

#[test]
fn test_init() {
    let dc = 0;
    let rst = 1;
    let busy = 2;

    let mut hal = Hal::new(&[
        // wakeup
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
        // sleep
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

    Display::new(
        hal.clone().spi(),
        hal.clone().pin(rst),
        hal.clone().pin(dc),
        hal.clone().pin(busy),
        hal.clone().delay(),
    )
    .init()
    .unwrap();

    hal.done();
}

#[test]
fn test_display() {
    let dc = 0;
    let rst = 1;
    let busy = 2;

    let mut hal = Hal::new(&[
        // wakeup
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
        // display
        Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
        Expectation::Spi(SpiTransaction::transaction_start()),
        Expectation::Spi(SpiTransaction::write(0x61)),
        Expectation::Spi(SpiTransaction::transaction_end()),
        Expectation::Digital(dc, PinTransaction::set(PinState::High)),
        Expectation::Spi(SpiTransaction::transaction_start()),
        Expectation::Spi(SpiTransaction::write_vec([0x02, 0x58, 0x01, 0xc0].to_vec())),
        Expectation::Spi(SpiTransaction::transaction_end()),
        Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
        Expectation::Spi(SpiTransaction::transaction_start()),
        Expectation::Spi(SpiTransaction::write(0x10)),
        Expectation::Spi(SpiTransaction::transaction_end()),
        Expectation::Digital(dc, PinTransaction::set(PinState::High)),
        Expectation::Spi(SpiTransaction::transaction_start()),
        Expectation::Spi(SpiTransaction::write_vec(
            [0b00010001; super::WIDTH * super::HEIGHT / 2].to_vec(),
        )),
        Expectation::Spi(SpiTransaction::transaction_end()),
        Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
        Expectation::Spi(SpiTransaction::transaction_start()),
        Expectation::Spi(SpiTransaction::write(0x04)),
        Expectation::Spi(SpiTransaction::transaction_end()),
        Expectation::Digital(busy, PinTransaction::get(PinState::High)),
        Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
        Expectation::Spi(SpiTransaction::transaction_start()),
        Expectation::Spi(SpiTransaction::write(0x12)),
        Expectation::Spi(SpiTransaction::transaction_end()),
        Expectation::Digital(busy, PinTransaction::get(PinState::Low)),
        Expectation::Digital(busy, PinTransaction::get(PinState::High)),
        Expectation::Digital(dc, PinTransaction::set(PinState::Low)),
        Expectation::Spi(SpiTransaction::transaction_start()),
        Expectation::Spi(SpiTransaction::write(0x04)),
        Expectation::Spi(SpiTransaction::transaction_end()),
        Expectation::Digital(busy, PinTransaction::get(PinState::High)),
        Expectation::Digital(busy, PinTransaction::get(PinState::High)),
        Expectation::Digital(busy, PinTransaction::get(PinState::Low)),
        Expectation::Delay(200_000_000),
        // sleep
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

    Display::new(
        hal.clone().spi(),
        hal.clone().pin(rst),
        hal.clone().pin(dc),
        hal.clone().pin(busy),
        hal.clone().delay(),
    )
    .display()
    .unwrap();

    hal.done();
}

#[test]
fn test_clear() {
    let mut hal = Hal::new(&[]);

    let mut spi = SpiMock::new(&[]);
    let mut dc = PinMock::new(&[]);
    let mut busy = PinMock::new(&[]);

    let mut epd = Display::new(
        spi.clone(),
        hal.clone().pin(0),
        dc.clone(),
        busy.clone(),
        hal.clone().delay(),
    );

    epd.buffer = [0xff; super::WIDTH * super::HEIGHT / 2];
    assert_eq!(epd.buffer, [0xff; super::WIDTH * super::HEIGHT / 2]);
    epd.clear().unwrap();
    assert_eq!(epd.buffer, [0b00010001; super::WIDTH * super::HEIGHT / 2]);

    spi.done();
    dc.done();
    busy.done();
    hal.done();
}

#[test]
fn test_set_pixel() {
    let mut hal = Hal::new(&[]);

    let mut spi = SpiMock::new(&[]);
    let mut dc = PinMock::new(&[]);
    let mut busy = PinMock::new(&[]);

    let mut epd = Display::new(
        spi.clone(),
        hal.clone().pin(0),
        dc.clone(),
        busy.clone(),
        hal.clone().delay(),
    );

    assert_eq!(epd.buffer, [0b00010001; super::WIDTH * super::HEIGHT / 2]);
    epd.set_pixel(0, 0, super::color::Color::BLACK).unwrap();
    epd.set_pixel(1, 0, super::color::Color::WHITE).unwrap();
    epd.set_pixel(2, 0, super::color::Color::GREEN).unwrap();
    epd.set_pixel(3, 0, super::color::Color::BLUE).unwrap();
    epd.set_pixel(4, 0, super::color::Color::RED).unwrap();
    epd.set_pixel(5, 0, super::color::Color::YELLOW).unwrap();
    epd.set_pixel(6, 0, super::color::Color::ORANGE).unwrap();
    assert_eq!(
        epd.buffer[0..4],
        [0b00000001, 0b00100011, 0b01000101, 0b01100001]
    );

    spi.done();
    dc.done();
    busy.done();
    hal.done();
}
