#[cfg(feature = "has_terminal")]
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

#[cfg(feature = "has_terminal")]
use tui::backend::CrosstermBackend;
#[cfg(feature = "has_terminal")]
use std::{
    io,
    time::{Duration, Instant},
};
use std::error::Error;
use tui::Terminal;

use training_mod_consts::*;

fn main() -> Result<(), Box<dyn Error>> {
    let menu;
    unsafe {
        menu = get_menu();
    }

    #[cfg(not(feature = "has_terminal"))] {
        let mut app = training_mod_tui::App::new(menu);
        let backend = tui::backend::TestBackend::new(100, 8);
        let mut terminal = Terminal::new(backend)?;
        let mut state = tui::widgets::ListState::default();
        state.select(Some(1));
        let mut url = String::new();
        let frame_res = terminal.draw(|f| url = training_mod_tui::ui(f, &mut app))?;

        for (i, cell) in frame_res.buffer.content().into_iter().enumerate() {
            print!("{}", cell.symbol);
            if i % frame_res.area.width as usize == frame_res.area.width as usize - 1 {
                print!("\n");
            }
        }
        println!();

        println!("URL: {}", url);
    }

    #[cfg(feature = "has_terminal")] {
        let app = training_mod_tui::App::new(menu);

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
        } else {
            println!("URL: {}", res.as_ref().unwrap());
        }
    }

    Ok(())
}

#[cfg(feature = "has_terminal")]
fn run_app<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: training_mod_tui::App,
    tick_rate: Duration,
) -> io::Result<String> {
    let mut last_tick = Instant::now();
    let mut url = String::new();
    loop {
        terminal.draw(|f| url = training_mod_tui::ui(f, &mut app).clone())?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));


        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(url),
                    KeyCode::Char('r') => app.on_r(),
                    KeyCode::Char('l') => app.on_l(),
                    KeyCode::Left => app.on_left(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Enter => app.on_a(),
                    KeyCode::Backspace => app.on_b(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            last_tick = Instant::now();
        }
    }
}
