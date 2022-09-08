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

fn test_backend_setup(ui_menu: UiMenu) -> Result<
    (Terminal<training_mod_tui::TestBackend>, training_mod_tui::App),
    Box<dyn Error>> {
    let app = training_mod_tui::App::new(ui_menu);
    let backend = tui::backend::TestBackend::new(75, 15);
    let terminal = Terminal::new(backend)?;
    let mut state = tui::widgets::ListState::default();
    state.select(Some(1));

    Ok((terminal, app))
}

#[test]
fn ensure_menu_retains_multi_selections() -> Result<(), Box<dyn Error>> {
    let menu;
    unsafe {
        menu = get_menu();
        println!("MENU.miss_tech_state: {}", MENU.miss_tech_state);
    }

    let (mut terminal, mut app) = test_backend_setup(menu)?;
    let mut json_response = String::new();
    let _frame_res = terminal.draw(|f| json_response = training_mod_tui::ui(f, &mut app))?;
    set_menu_from_json(json_response);
    unsafe {
        // At this point, we didn't change the menu at all; we should still see all missed tech flags.
        assert_eq!(MENU.miss_tech_state,
                   MissTechFlags::all());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let menu;
    unsafe {
        menu = get_menu();
    }

    #[cfg(not(feature = "has_terminal"))] {
        let (mut terminal, mut app) = test_backend_setup(menu)?;
        let mut json_response = String::new();
        let frame_res = terminal.draw(|f| json_response = training_mod_tui::ui(f, &mut app))?;

        for (i, cell) in frame_res.buffer.content().iter().enumerate() {
            print!("{}", cell.symbol);
            if i % frame_res.area.width as usize == frame_res.area.width as usize - 1 {
                println!();
            }
        }
        println!();

        println!("json_response:\n{}", json_response);
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
    let mut json_response = String::new();
    loop {
        terminal.draw(|f| json_response = training_mod_tui::ui(f, &mut app).clone())?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(json_response),
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
            last_tick = Instant::now();
        }
    }
}
