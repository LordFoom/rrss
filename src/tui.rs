use std::{
    collections::HashMap,
    io::{self, stdout, Stdout},
    thread::{self, spawn},
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
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::{
    api::fetch_rss_feed,
    config::{save_config, RssConfig},
    model::{App, AppState, SelectedPane, StatefulItemList},
};

const _TODO_HEADER_BG: Color = tailwind::BLUE.c950;
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
    let [channels_pane, items_pane] = sidebar.areas(left);
    let [content_pane] = content.areas(right);

    let header_block = Block::new()
        .title("RRSS")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .style(Style::default().fg(Color::Rgb(212, 144, 29)));

    let header = Paragraph::new(
        r"RRSS rss reader
        [R]efresh channnel | [S]ave channels",
    )
    .block(header_block);

    frame.render_widget(header, top);

    display_channels(frame, app, channels_pane)?;

    display_selected_channel_items(frame, app, items_pane)?;

    let item_content = app.content_pane_text();
    display_selected_item(frame, &item_content, content_pane)?;

    if let Some(text) = app.info_popup_text.clone() {
        show_info_popup(&text, frame);
    }

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
                .bg(ALT_ROW_COLOR)
                .fg(SELECTED_STYLE_FG)
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
fn display_selected_channel_items(frame: &mut Frame, app: &mut App, item_pane: Rect) -> Result<()> {
    let bt = get_border_type(app.selected_pane == SelectedPane::Items);
    let items_block = Block::new()
        .title("Items")
        .borders(Borders::ALL)
        .border_type(bt)
        .style(Style::default().fg(TEXT_COLOR));
    //TODO here we gonna stick in the items we got oh yeah
    let item_list = if let Some(channel) = app.get_selected_channel() {
        if app.construct_items {
            //TODO here we can do a fetch
            app.current_items = StatefulItemList::from(channel);
            app.construct_items = false;
        }
        let items: Vec<ListItem> = app
            .current_items
            .items
            .clone()
            .iter()
            .map(|item| ListItem::new(item.get_title()))
            .collect();
        List::new(items).block(items_block).highlight_style(
            Style::default()
                .bg(NORMAL_ROW_COLOR)
                .fg(SELECTED_STYLE_FG)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        let li = ["We are default items"];
        List::new(li).block(items_block)
    };

    frame.render_stateful_widget(item_list, item_pane, &mut app.current_items.state);
    Ok(())
}

///Display the content for the selected item in its pane
fn display_selected_item(frame: &mut Frame, text: &str, item_pane: Rect) -> Result<()> {
    let view_block = Block::new()
        .title("Content")
        .borders(Borders::all())
        .border_type(BorderType::Thick)
        .style(Style::default().fg(Color::Cyan));
    let item_content = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(view_block);
    frame.render_widget(item_content, item_pane);
    Ok(())
}

///Run run run the app merrily down the bitstream
pub async fn run_app<B: Backend>(term: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        term.draw(|f| ui(f, app).expect("Could not draw the ui"))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => app.state = AppState::Stopped,
                //todo differentiate between the different selected states
                KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => app.select_down(),
                KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => app.select_up(),
                KeyCode::Char('r') | KeyCode::Char('R') => reload_selected_channel(app).await?,
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    app.info_popup_text = Some("Saving config...".to_string());
                    save_into_config(app).await?;
                }
                KeyCode::Tab => app.change_selected_pane(),
                _ => {}
            }
        }
        if app.state == AppState::Stopped {
            return Ok(());
        }
    }
}

pub async fn reload_selected_channel(app: &mut App) -> Result<()> {
    //get the selected channel, if it exists
    //TODO mark app state to show loading popup
    if let Some(selected_channel) = app.get_selected_channel() {
        if let Some(channel) = fetch_rss_feed(&selected_channel.get_link()).await? {
            app.update_selected_channel(&channel);
        };
    }
    Ok(())
}

pub async fn save_into_config(app: &mut App) -> Result<()> {
    app.info_popup_text = Some("Saving config...".to_string());
    let mut channels = HashMap::new();
    for channel in app.channels.channels.clone() {
        channels.insert(channel.title.clone(), channel.get_link().clone());
    }
    let cfg = RssConfig { channels };
    //TODO make this take in a file path
    thread::spawn(|| save_config(None, cfg));
    Ok(())
}

pub fn show_info_popup(txt: &str, f: &mut Frame) {
    let popup_block = Block::new()
        .style(Style::default().fg(Color::Rgb(190, 147, 228)))
        .borders(Borders::all())
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Rgb(191, 0, 255)));
    let popup_paragraph = Paragraph::new(txt).block(popup_block);
    let centered_pane = centered_rect(20, 20, f.size());
    f.render_widget(popup_paragraph, centered_pane);
}

///Get an area that is centered'ish - with horizontal and vertical bias
///In which one could for example display a popup
fn centered_rect(h: u16, v: u16, rect: Rect) -> Rect {
    //cut into 3 vertical rows
    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage((100 - v) / 2),
            Constraint::Percentage(v),
            Constraint::Percentage((100 - v) / 2),
        ],
    )
    .split(rect);

    //now we split the middle vertical block into 3 columns
    //and we return the middle column
    Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage((100 - h) / 2),
            Constraint::Percentage(h),
            Constraint::Percentage((100 - h) / 2),
        ],
    )
    .split(layout[1])[1]
}
