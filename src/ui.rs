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
    boards::{self, Boards},
    game::Game,
    inputs::{events::Events, InputEvent, Key},
};

use crate::GOLError;

pub fn start_ui(game: &mut Game, sleep_time: u64) -> Result<(), GOLError> {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let tick_rate = Duration::from_millis(if sleep_time < 10 { 10 } else { sleep_time });
    let events = Events::new(tick_rate);

    loop {
        terminal.draw(|pg| draw(pg, &game)).unwrap();

        let result = match events.next().unwrap_or(InputEvent::Tick) {
            InputEvent::Input(key) => process_key(key),
            InputEvent::Tick => update(game),
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

fn draw<B>(item: &mut Frame<B>, game: &Game)
where
    B: Backend,
{
    let size = item.size();

    check_size(&size, game);

    let title = draw_title();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(size);

    let bod = draw_body(game);

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

fn draw_body<'a>(game: &Game) -> Paragraph<'a> {
    match &game.board {
        boards::Boards::AntBoard(ant) => Paragraph::new(ant.render()),
        boards::Boards::GolBoard(gol) => Paragraph::new(gol.render()),
    }
    .alignment(Alignment::Center)
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

fn update(game: &mut Game) -> Return {
    match &mut game.board {
        Boards::GolBoard(board) => match board.method {
            GOLMethod::Normal => *board = board.next_state(),
            GOLMethod::VonNeumann => *board = board.next_state_neumann(),
        },
        Boards::AntBoard(board) => *board = board.update(),
    };

    Return::Continue
}

fn check_size(rect: &Rect, game: &Game) {
    let (width, height) = match &game.board {
        boards::Boards::AntBoard(ant) => (ant.width, ant.height),
        boards::Boards::GolBoard(gol) => (gol.width, gol.height),
    };

    if (rect.width as usize) < width + 5 {
        panic!(
            "Require width >= to the board width + 5, (board {}, terminal {})",
            width, rect.width
        );
    }

    if (rect.height as usize) < height + 5 {
        panic!(
            "Require height >= to the board height + 5, (board {}, terminal {})",
            height, rect.height
        );
    }
}
