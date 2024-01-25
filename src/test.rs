extern crate std;
use super::*;
use embedded_hal_mock::eh1::{pin::State as PinState, top_level::Hal};

#[test]
fn test_reset_panel() {
    let mut hal = Hal::new(&[]);

    let spi = hal.clone().spi();
    let dc = hal.clone().pin();
    let busy = hal.clone().pin();
    let rst = hal.clone().pin();
    let delay = hal.clone().delay();

    hal.update_expectations(&[
        rst.expect_set(PinState::Low),
        delay.expect_delay_ns(1_000_000),
        rst.expect_set(PinState::High),
        delay.expect_delay_ns(200_000_000),
    ]);

    let mut epd = Display::new(spi, rst, dc, busy, delay);

    epd.reset_panel().unwrap();
    hal.done();
}

#[test]
fn test_send_command() {
    let mut hal = Hal::new(&[]);

    let spi = hal.clone().spi();
    let rst = hal.clone().pin();
    let dc = hal.clone().pin();
    let busy = hal.clone().pin();
    let delay = hal.clone().delay();

    hal.update_expectations(&[
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(23),
        spi.expect_transaction_end(),
    ]);

    let mut epd = Display::new(spi, rst, dc, busy, delay);

    epd.send_command(23).unwrap();

    hal.done();
}

#[test]
fn test_send_data() {
    let mut hal = Hal::new(&[]);

    let spi = hal.clone().spi();
    let rst = hal.clone().pin();
    let dc = hal.clone().pin();
    let busy = hal.clone().pin();
    let delay = hal.clone().delay();

    hal.update_expectations(&[
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([20, 45].to_vec()),
        spi.expect_transaction_end(),
    ]);

    let mut epd = Display::new(spi, rst, dc, busy, delay);

    epd.send_data(&[20, 45]).unwrap();

    hal.done();
}

#[test]
fn test_sleep_panel() {
    let mut hal = Hal::new(&[]);

    let spi = hal.clone().spi();
    let rst = hal.clone().pin();
    let dc = hal.clone().pin();
    let busy = hal.clone().pin();
    let delay = hal.clone().delay();

    hal.update_expectations(&[
        delay.expect_delay_ns(10_000_000),
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x07),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write(0xA5),
        spi.expect_transaction_end(),
        delay.expect_delay_ns(100_000_000),
        rst.expect_set(PinState::Low),
        dc.expect_set(PinState::Low),
    ]);

    Display::new(spi, rst, dc, busy, delay).sleep().unwrap();

    hal.done();
}

#[test]
fn test_wakeup() {
    let mut hal = Hal::new(&[]);

    let spi = hal.clone().spi();
    let rst = hal.clone().pin();
    let dc = hal.clone().pin();
    let busy = hal.clone().pin();
    let delay = hal.clone().delay();

    hal.update_expectations(&[
        rst.expect_set(PinState::Low),
        delay.expect_delay_ns(1_000_000),
        rst.expect_set(PinState::High),
        delay.expect_delay_ns(200_000_000),
        // busy
        // busy
        // no longer busy
        busy.expect_get(PinState::Low),
        busy.expect_get(PinState::Low),
        busy.expect_get(PinState::High),
        // self.send_command(registers::PANEL_SET_REGISTER)?;
        // self.send_data(&[0xEF, 0x08])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x00),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0xEF, 0x08].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::POWER_SET_REGISTER)?;
        // self.send_data(&[0x37, 0x00, 0x05, 0x05])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x01),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x37, 0x00, 0x05, 0x05].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::POWER_OFF_SEQ_SET_REGISTER)?;
        // self.send_data(&[0x00])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x03),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x00].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::BOOSTER_SOFTSTART_REGISTER)?;
        // self.send_data(&[0xC7, 0xC7, 0x1D])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x06),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0xC7, 0xC7, 0x1D].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::TEMP_SENSOR_EN_REGISTER)?;
        // self.send_data(&[0x00])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x41),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x00].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)?;
        // self.send_data(&[0x37])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x50),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x37].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(0x60)?;
        // self.send_data(&[0x20])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x60),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x20].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::RESOLUTION_SET_REGISTER)?;
        // self.send_data(&[0x02, 0x58, 0x01, 0xC0])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x61),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x02, 0x58, 0x01, 0xC0].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(0xE3)?;
        // self.send_data(&[0xAA])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0xE3),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0xAA].to_vec()),
        spi.expect_transaction_end(),
        // self.delay.delay_ms(100u32);
        // self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)?;
        // self.send_data(&[0x37])?;
        delay.expect_delay_ns(100_000_000),
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x50),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x37].to_vec()),
        spi.expect_transaction_end(),
    ]);

    Display::new(spi, rst, dc, busy, delay).wakeup().unwrap();

    hal.done();
}

#[test]
fn test_init() {
    let mut hal = Hal::new(&[]);

    let spi = hal.clone().spi();
    let rst = hal.clone().pin();
    let dc = hal.clone().pin();
    let busy = hal.clone().pin();
    let delay = hal.clone().delay();

    hal.update_expectations(&[
        // wakeup
        rst.expect_set(PinState::Low),
        delay.expect_delay_ns(1_000_000),
        rst.expect_set(PinState::High),
        delay.expect_delay_ns(200_000_000),
        // busy
        // busy
        // no longer busy
        busy.expect_get(PinState::Low),
        busy.expect_get(PinState::Low),
        busy.expect_get(PinState::High),
        // self.send_command(registers::PANEL_SET_REGISTER)?;
        // self.send_data(&[0xEF, 0x08])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x00),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0xEF, 0x08].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::POWER_SET_REGISTER)?;
        // self.send_data(&[0x37, 0x00, 0x05, 0x05])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x01),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x37, 0x00, 0x05, 0x05].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::POWER_OFF_SEQ_SET_REGISTER)?;
        // self.send_data(&[0x00])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x03),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x00].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::BOOSTER_SOFTSTART_REGISTER)?;
        // self.send_data(&[0xC7, 0xC7, 0x1D])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x06),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0xC7, 0xC7, 0x1D].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::TEMP_SENSOR_EN_REGISTER)?;
        // self.send_data(&[0x00])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x41),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x00].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)?;
        // self.send_data(&[0x37])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x50),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x37].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(0x60)?;
        // self.send_data(&[0x20])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x60),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x20].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::RESOLUTION_SET_REGISTER)?;
        // self.send_data(&[0x02, 0x58, 0x01, 0xC0])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x61),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x02, 0x58, 0x01, 0xC0].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(0xE3)?;
        // self.send_data(&[0xAA])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0xE3),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0xAA].to_vec()),
        spi.expect_transaction_end(),
        // self.delay.delay_ms(100u32);
        // self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)?;
        // self.send_data(&[0x37])?;
        delay.expect_delay_ns(100_000_000),
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x50),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x37].to_vec()),
        spi.expect_transaction_end(),
        // sleep
        delay.expect_delay_ns(10_000_000),
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x07),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write(0xA5),
        spi.expect_transaction_end(),
        delay.expect_delay_ns(100_000_000),
        rst.expect_set(PinState::Low),
        dc.expect_set(PinState::Low),
    ]);

    Display::new(spi, rst, dc, busy, delay).init().unwrap();

    hal.done();
}

#[test]
fn test_display() {
    let mut hal = Hal::new(&[]);

    let spi = hal.clone().spi();
    let rst = hal.clone().pin();
    let dc = hal.clone().pin();
    let busy = hal.clone().pin();
    let delay = hal.clone().delay();

    hal.update_expectations(&[
        // wakeup
        rst.expect_set(PinState::Low),
        delay.expect_delay_ns(1_000_000),
        rst.expect_set(PinState::High),
        delay.expect_delay_ns(200_000_000),
        // busy
        // busy
        // no longer busy
        busy.expect_get(PinState::Low),
        busy.expect_get(PinState::Low),
        busy.expect_get(PinState::High),
        // self.send_command(registers::PANEL_SET_REGISTER)?;
        // self.send_data(&[0xEF, 0x08])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x00),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0xEF, 0x08].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::POWER_SET_REGISTER)?;
        // self.send_data(&[0x37, 0x00, 0x05, 0x05])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x01),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x37, 0x00, 0x05, 0x05].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::POWER_OFF_SEQ_SET_REGISTER)?;
        // self.send_data(&[0x00])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x03),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x00].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::BOOSTER_SOFTSTART_REGISTER)?;
        // self.send_data(&[0xC7, 0xC7, 0x1D])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x06),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0xC7, 0xC7, 0x1D].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::TEMP_SENSOR_EN_REGISTER)?;
        // self.send_data(&[0x00])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x41),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x00].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)?;
        // self.send_data(&[0x37])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x50),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x37].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(0x60)?;
        // self.send_data(&[0x20])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x60),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x20].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(registers::RESOLUTION_SET_REGISTER)?;
        // self.send_data(&[0x02, 0x58, 0x01, 0xC0])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x61),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x02, 0x58, 0x01, 0xC0].to_vec()),
        spi.expect_transaction_end(),
        // self.send_command(0xE3)?;
        // self.send_data(&[0xAA])?;
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0xE3),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0xAA].to_vec()),
        spi.expect_transaction_end(),
        // self.delay.delay_ms(100u32);
        // self.send_command(registers::VCOM_DATA_INTERVAL_REGISTER)?;
        // self.send_data(&[0x37])?;
        delay.expect_delay_ns(100_000_000),
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x50),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x37].to_vec()),
        spi.expect_transaction_end(),
        // display
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x61),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0x02, 0x58, 0x01, 0xc0].to_vec()),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x10),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write_vec([0b00010001; super::WIDTH * super::HEIGHT / 2].to_vec()),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x04),
        spi.expect_transaction_end(),
        busy.expect_get(PinState::High),
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x12),
        spi.expect_transaction_end(),
        busy.expect_get(PinState::Low),
        busy.expect_get(PinState::High),
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x04),
        spi.expect_transaction_end(),
        busy.expect_get(PinState::High),
        busy.expect_get(PinState::High),
        busy.expect_get(PinState::Low),
        delay.expect_delay_ns(200_000_000),
        // sleep
        delay.expect_delay_ns(10_000_000),
        dc.expect_set(PinState::Low),
        spi.expect_transaction_start(),
        spi.expect_write(0x07),
        spi.expect_transaction_end(),
        dc.expect_set(PinState::High),
        spi.expect_transaction_start(),
        spi.expect_write(0xA5),
        spi.expect_transaction_end(),
        delay.expect_delay_ns(100_000_000),
        rst.expect_set(PinState::Low),
        dc.expect_set(PinState::Low),
    ]);

    Display {
        spi,
        rst,
        dc,
        busy,
        delay,
        buffer: [0b0001_0001; (WIDTH * HEIGHT) / 2],
    }
    .display()
    .unwrap();

    hal.done();
}

#[test]
fn test_set_pixel() {
    let mut hal = Hal::new(&[]);

    let spi = hal.clone().spi();
    let rst = hal.clone().pin();
    let dc = hal.clone().pin();
    let busy = hal.clone().pin();
    let delay = hal.clone().delay();

    let mut epd = Display::new(spi, rst, dc, busy, delay);

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

    hal.done();
}
