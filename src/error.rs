#[derive(Debug, PartialEq)]
pub enum Error<BUSY, RST, DC, S>
where
    BUSY: embedded_hal::digital::Error,
    RST: embedded_hal::digital::Error,
    DC: embedded_hal::digital::Error,
    S: embedded_hal::spi::Error,
{
    BusyPin(BUSY),
    DataCommandPin(DC),
    ResetPin(RST),
    Spi(S),
}
