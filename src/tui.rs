use std::io::{self, Stdout};

use anyhow::{Context, Result};
use crossterm::{
    event::EnableMouseCapture, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    prelude::*,
    style::palette::tailwind,
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

use crate::model::{App, AppState};

const TODO_HEADER_BG: Color = tailwind::BLUE.c950;
const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const ALT_ROW_COLOR: Color = tailwind::SLATE.c900;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    let term = enable_raw_mode().context("Unable to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).context("Alternate screen switch...FAILED")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("Could not create the terminal")
}

pub fn restore_terminal(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("Unable to disable raw mode")?;
    execute!(term.backend_mut(), LeaveAlternateScreen)
        .context("Unable to return to main screen")?;
    term.show_cursor().context("Could not reveal cursor")
}

pub fn run_app(term: &mut Terminal<impl Backend>, app: &mut App) {
    while app.state == AppState::RUNNING {}
}

///Sets up the ui and returns the 4 components
///Top bar
///Main area which has left bar and main concat_idents!(
///left bar has channel and below it items
///)
pub fn ui(frame: &mut Frame, app: &mut App) -> Result<()> {
    let vertical = Layout::vertical([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);
    let horizontal = Layout::horizontal([Constraint::Ratio(1, 5), Constraint::Ratio(4, 5)]);
    let sidebar = Layout::vertical([Constraint::Ratio(1, 5), Constraint::Ratio(4, 5)]);
    let content = Layout::horizontal([Constraint::Fill(1)]);

    let [top, bottom] = vertical.areas(frame.size());
    let [left, right] = horizontal.areas(bottom);
    let [channel_pane, item_pane] = sidebar.areas(left);
    let [content_pane] = content.areas(item_pane);


    let header_block

    let channel_block = Block::new().title("Channels").borders(Borders::all())
    .style();

    let channel_items: Vec<ListItem> = app
        .channels
        .channels
        .iter()
        .map(|chnl| ListItem::new(chnl.title.clone()))
        .collect();

    let channel_list = List::new(channel_items)
        .block(channel_block)
        .highlight_symbol(">")
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_stateful_widget(channel_list, channel_pane, &mut app.channels.state);
    //do i return the bits to populate or do i poplate them in here?
    Ok(())
}
