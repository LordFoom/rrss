use anyhow::Result;
use crossterm::terminal::enable_raw_mode;

pub fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    Ok(())
}
