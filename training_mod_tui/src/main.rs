#[cfg(feature = "has_terminal")]
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

#[cfg(feature = "has_terminal")]
use tui::backend::CrosstermBackend;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use std::borrow::Borrow;
use serde_json::Value;
use tui::{
    backend::{Backend, TestBackend},
    layout::{Constraint, Corner, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use tui::buffer::Buffer;

use training_mod_consts::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = training_mod_tui::App::new();
    let menu;
    unsafe {
        menu = get_menu();
    }

    let mut items = Vec::new();
    for sub_menu in menu.sub_menus.iter() {
        items.push((sub_menu.title, sub_menu.help_text));
    }
    app.items = training_mod_tui::StatefulList::with_items(items);

    #[cfg(not(feature = "has_terminal"))] {
        let backend = TestBackend::new(100, 8);
        let mut terminal = Terminal::new(backend)?;
        let mut state = ListState::default();
        state.select(Some(1));
        let frame_res = terminal.draw(|f| training_mod_tui::ui(f, &mut app))?;

        for (i, cell) in frame_res.buffer.content().into_iter().enumerate() {
            print!("{}", cell.symbol);
            if i % frame_res.area.width as usize == frame_res.area.width as usize - 1 {
                print!("\n");
            }
        }
        println!();
    }

    #[cfg(feature = "has_terminal")] {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let tick_rate = Duration::from_millis(250);
        let res = run_app(&mut terminal, app, tick_rate);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err)
        }
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: training_mod_tui::App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut state = ListState::default();
    state.select(Some(1));
    loop {
        let frame_res = terminal.draw(|f| training_mod_tui::ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        #[cfg(feature = "has_terminal")]
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => app.items.unselect(),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    KeyCode::Enter => {
                        for (i, cell) in frame_res.buffer.content().into_iter().enumerate() {
                            print!("{}", cell.symbol);
                            if i % frame_res.area.width as usize == frame_res.area.width as usize - 1 {
                                print!("\n");
                            }
                        }
                        println!();
                    },
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}