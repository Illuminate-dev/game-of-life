use std::{io, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

use crate::{
    args::GOLMethod,
    board::{self, Board},
    inputs::{events::Events, InputEvent, Key},
};

use crate::GOLError;

pub fn start_ui(board: &mut Board, config: super::Config) -> Result<(), GOLError> {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let tick_rate = Duration::from_millis(if config.sleep_time < 10 {
        10
    } else {
        config.sleep_time
    });
    let events = Events::new(tick_rate);

    loop {
        terminal.draw(|pg| draw(pg, &board)).unwrap();

        let result = match events.next().unwrap_or(InputEvent::Tick) {
            InputEvent::Input(key) => process_key(key),
            InputEvent::Tick => update(board, config.neighbor_method),
        };

        if result == Return::Exit {
            break;
        }
    }

    disable_raw_mode().unwrap();

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    terminal.show_cursor().unwrap();

    Ok(())
}

fn draw<B>(item: &mut Frame<B>, board: &Board)
where
    B: Backend,
{
    let size = item.size();

    check_size(&size, board);

    let title = draw_title();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(size);

    let bod = draw_body(board);

    item.render_widget(title, chunks[0]);

    item.render_widget(bod, chunks[1]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Conway's Game of Life")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn draw_body<'a>(board: &Board) -> Paragraph<'a> {
    Paragraph::new(board.render()).alignment(Alignment::Center)
}

#[derive(PartialEq, Eq)]
pub enum Return {
    Exit,
    Continue,
}

fn process_key(key: Key) -> Return {
    if key == Key::Ctrl('c') || key == Key::Char('q') {
        return Return::Exit;
    }
    Return::Continue
}

fn update(board: &mut Board, method: GOLMethod) -> Return {
    let neighbor_function = match method {
        GOLMethod::Normal => board::Board::next_state,
        GOLMethod::VonNeumann => board::Board::next_state_neumann,
    };

    *board = neighbor_function(board);

    Return::Continue
}

fn check_size(rect: &Rect, board: &Board) {
    if (rect.width as usize) < board.width + 5 {
        panic!(
            "Require width >= to the board width + 5, (board {}, terminal {})",
            board.width, rect.width
        );
    }

    if (rect.height as usize) < board.height + 5 {
        panic!(
            "Require height >= to the board height + 5, (board {}, terminal {})",
            board.height, rect.height
        );
    }
}
