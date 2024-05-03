use std::io::{self, Stdout};

use anyhow::{Context, Result};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    Frame, Terminal,
};

use crate::model::{App, AppState};

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    let term = enable_raw_mode().context("Unable to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("Alternate screen switch...FAILED")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("Could not create the terminal")
}

pub fn restore_terminal(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("Unable to disable raw mode")?;
    execute!(term.backend_mut(), LeaveAlternateScreen)
        .context("Unable to return to main screen")?;
    term.show_cursor().context("Could not reveal cursor")
}

pub fn run_app(term: &mut Terminal<impl Backend>, app: &mut App) {
    while app.state == AppState::RUNNING {

        //we want to display the channels to start with
    }
}

///Sets up the ui and returns the 4 components
///Top bar
///Main area which has left bar and main concat_idents!(
///left bar has channel and below it items
///)
pub fn ui(frame: &mut Frame, app: &App) -> Result<()> {
    let vertical = Layout::vertical([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);
    let horizontal = Layout::horizontal([Constraint::Ratio(1, 5), Constraint::Ratio(4, 5)]);
    let sidebar = Layout::vertical([Constraint::Ratio(1, 5), Constraint::Ratio(4, 5)]);
    let content = Layout::horizontal([Constraint::Fill(1)]);

    //do i return the bits to populate or do i poplate them in here?
    Ok(())
}
