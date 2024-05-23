use std::{
    fmt::write,
    io::{self, stdout, Stdout},
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    prelude::*,
    style::palette::tailwind,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::{
    display::display_channel,
    model::{App, AppState, SelectedPane},
};

const TODO_HEADER_BG: Color = tailwind::BLUE.c950;
const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const ALT_ROW_COLOR: Color = tailwind::SLATE.c900;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("Unable to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Alternate screen switch...FAILED")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("Could not create the terminal")
}

pub fn restore_terminal() -> Result<()> {
    disable_raw_mode().context("Unable to disable raw mode")?;
    stdout().execute(LeaveAlternateScreen)?;
    // execute!(term.backend_mut(), LeaveAlternateScreen)
    //     .context("Unable to return to main screen")?;
    // term.show_cursor().context("Could not reveal cursor")
    Ok(())
}

///Sets up the ui and returns the 4 components
///Top bar
///Main area which has left bar and main concat_idents!(
///left bar has channel and below it items
///)
pub fn ui(frame: &mut Frame, app: &mut App) -> Result<()> {
    let vertical = Layout::vertical([Constraint::Ratio(1, 8), Constraint::Ratio(7, 8)]);
    let horizontal = Layout::horizontal([Constraint::Ratio(1, 5), Constraint::Ratio(4, 5)]);
    let sidebar = Layout::vertical([Constraint::Ratio(1, 5), Constraint::Ratio(4, 5)]);
    let content = Layout::horizontal([Constraint::Fill(1)]);

    let [top, bottom] = vertical.areas(frame.size());
    let [left, right] = horizontal.areas(bottom);
    let [channel_pane, item_pane] = sidebar.areas(left);
    let [content_pane] = content.areas(right);

    let header_block = Block::new()
        .title("RRSS")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .style(Style::default().fg(Color::DarkGray));

    let header = Paragraph::new("RRSS rss reader").block(header_block);
    frame.render_widget(header, top);

    display_channels(frame, app, channel_pane)?;

    display_items(frame, app, item_pane)?;

    let view_block = Block::new()
        .title("Content")
        .borders(Borders::all())
        .border_type(BorderType::Thick)
        .style(Style::default().fg(Color::Cyan));
    let item_content = Paragraph::new("The content lorem dorem ipsum galactum").block(view_block);
    frame.render_widget(item_content, content_pane);

    Ok(())
}

///Show the channels we are monitoring in their pane
fn display_channels(frame: &mut Frame, app: &mut App, channel_pane: Rect) -> Result<()> {
    let bt = get_border_type(app.selected_pane == SelectedPane::Channels);

    let channel_block = Block::new()
        .title("Channels")
        .borders(Borders::all())
        .border_type(bt)
        .style(Style::default().fg(Color::Yellow));

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
    Ok(())
}

fn get_border_type(selected: bool) -> BorderType {
    if selected {
        BorderType::Thick
    } else {
        BorderType::Plain
    }
}

///Display the items for the selected channel in their pane
fn display_items(frame: &mut Frame, app: &mut App, item_pane: Rect) -> Result<()> {
    let bt = get_border_type(app.selected_pane == SelectedPane::Items);
    let items_block = Block::new()
        .title("Items")
        .borders(Borders::ALL)
        .border_type(bt)
        .style(Style::default().fg(Color::Yellow));
    //TODO here we gonna stick in the items we got oh yeah
    let item_list = if let Some(channel) = app.get_selected_channel() {
        if app.construct_items {
            //TODO here we can do a fetch
        }
        let items: Vec<ListItem> = channel
            .items
            .clone()
            .iter()
            .map(|item| ListItem::new(item.get_title()))
            .collect();
        List::new(items).block(items_block)
    } else {
        let li = ["We are default items"];
        List::new(li).block(items_block)
    };

    frame.render_stateful_widget(item_list, item_pane, &mut app.current_items.state);
    Ok(())
}

///Run run run the app merrily down the bitstream
pub fn run_app<B: Backend>(term: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        term.draw(|f| ui(f, app).expect("Could not draw the ui"))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => app.state = AppState::Stopped,
                //todo differentiate between the different selected states
                KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => app.select_down(),
                KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => app.select_up(),
                KeyCode::Tab => app.change_selected_pane(),
                _ => {}
            }
        }
        if app.state == AppState::Stopped {
            return Ok(());
        }
    }
}
