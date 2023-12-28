#![allow(dead_code)]
pub(super) const PANEL_SET_REGISTER: u8 = 0x00;
pub(super) const POWER_SET_REGISTER: u8 = 0x01;
pub(super) const VCM_DC_SET_REGISTER: u8 = 0x02;
pub(super) const POWER_OFF_SEQ_SET_REGISTER: u8 = 0x03;
pub(super) const POWER_OFF_REGISTER: u8 = 0x04;
pub(super) const BOOSTER_SOFTSTART_REGISTER: u8 = 0x06;
pub(super) const DEEP_SLEEP_REGISTER: u8 = 0x07;

pub(super) const DATA_START_TRANS_REGISTER: u8 = 0x10;
pub(super) const DATA_STOP_REGISTER: u8 = 0x11;
pub(super) const DISPLAY_REF_REGISTER: u8 = 0x12;
pub(super) const IMAGE_PROCESS_REGISTER: u8 = 0x13;

pub(super) const PLL_CONTROL_REGISTER: u8 = 0x30;

pub(super) const TEMP_SENSOR_REGISTER: u8 = 0x40;
pub(super) const TEMP_SENSOR_EN_REGISTER: u8 = 0x41;
pub(super) const TEMP_SENSOR_WR_REGISTER: u8 = 0x42;
pub(super) const TEMP_SENSOR_RD_REGISTER: u8 = 0x43;

pub(super) const VCOM_DATA_INTERVAL_REGISTER: u8 = 0x50;
pub(super) const LOW_POWER_DETECT_REGISTER: u8 = 0x51;
pub(super) const RESOLUTION_SET_REGISTER: u8 = 0x61;
pub(super) const STATUS_REGISTER: u8 = 0x71;
pub(super) const VCOM_VALUE_REGISTER: u8 = 0x81;
