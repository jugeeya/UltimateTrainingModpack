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

fn test_backend_setup<'a>(ui_menu: UiMenu<'a>, menu_defaults: (UiMenu<'a>, String)) -> Result<
    (Terminal<training_mod_tui::TestBackend>, training_mod_tui::App<'a>),
    Box<dyn Error>> {
    let app = training_mod_tui::App::<'a>::new(ui_menu, menu_defaults);
    let backend = tui::backend::TestBackend::new(75, 15);
    let terminal = Terminal::new(backend)?;
    let mut state = tui::widgets::ListState::default();
    state.select(Some(1));

    Ok((terminal, app))
}

#[test]
fn test_set_airdodge() -> Result<(), Box<dyn Error>> {
    let menu;
    let mut prev_menu;
    let menu_defaults;
    unsafe {
        prev_menu = MENU.clone();
        menu = ui_menu(MENU);
        menu_defaults = (ui_menu(MENU), serde_json::to_string(&MENU).unwrap());
    }

    let (_terminal, mut app) = test_backend_setup(menu, menu_defaults)?;
    // Enter Mash Toggles
    app.on_a();
    // Set Mash Airdodge
    app.on_a();
    let menu_json = app.get_menu_selections();
    let menu_struct = serde_json::from_str::<MenuJsonStruct>(&menu_json).unwrap();
    let menu = menu_struct.menu;
    let _ = menu_struct.defaults_menu;
    prev_menu.mash_state.toggle(Action::AIR_DODGE);
    assert_eq!(
        serde_json::to_string(&prev_menu).unwrap(),
        serde_json::to_string(&menu).unwrap()
    );

    Ok(())
}

#[test]
fn test_ensure_menu_retains_selections() -> Result<(), Box<dyn Error>> {
    let menu;
    let prev_menu;
    let menu_defaults;
    unsafe {
        prev_menu = MENU.clone();
        menu = ui_menu(MENU);
        menu_defaults = (ui_menu(MENU), serde_json::to_string(&MENU).unwrap());
    }

    let (_terminal, app) = test_backend_setup(menu, menu_defaults)?;
    let menu_json = app.get_menu_selections();
    let menu_struct = serde_json::from_str::<MenuJsonStruct>(&menu_json).unwrap();
    let menu = menu_struct.menu;
    let _ = menu_struct.defaults_menu;
    // At this point, we didn't change the menu at all; we should still see all the same options.
    assert_eq!(
        serde_json::to_string(&prev_menu).unwrap(),
        serde_json::to_string(&menu).unwrap()
    );

    Ok(())
}

#[test]
fn test_save_and_reset_defaults() -> Result<(), Box<dyn Error>> {
    let menu;
    let mut prev_menu;
    let menu_defaults;
    unsafe {
        prev_menu = MENU.clone();
        menu = ui_menu(MENU);
        menu_defaults = (ui_menu(MENU), serde_json::to_string(&MENU).unwrap());
    }

    let (_terminal, mut app) = test_backend_setup(menu, menu_defaults)?;

    // Enter Mash Toggles
    app.on_a();
    // Set Mash Airdodge
    app.on_a();
    // Return to submenu selection
    app.on_b();
    // Save Defaults
    app.on_x();
    // Enter Mash Toggles again
    app.on_a();
    // Unset Mash Airdodge
    app.on_a();
    // Return to submenu selection
    app.on_b();

    let menu_json = app.get_menu_selections();
    let menu_struct = serde_json::from_str::<MenuJsonStruct>(&menu_json).unwrap();
    let menu = menu_struct.menu;
    let defaults_menu = menu_struct.defaults_menu;

    assert_eq!(
        serde_json::to_string(&prev_menu).unwrap(),
        serde_json::to_string(&menu).unwrap(),
        "The menu should be the same as how we started"
    );
    prev_menu.mash_state.toggle(Action::AIR_DODGE);
    assert_eq!(
        serde_json::to_string(&prev_menu).unwrap(),
        serde_json::to_string(&defaults_menu).unwrap(),
        "The defaults menu should have Mash Airdodge"
    );

    // Reset all to defaults
    app.on_r();
    let menu_json = app.get_menu_selections();
    let menu_struct = serde_json::from_str::<MenuJsonStruct>(&menu_json).unwrap();
    let menu = menu_struct.menu;
    let _ = menu_struct.defaults_menu;

    assert_eq!(
        serde_json::to_string(&prev_menu).unwrap(),
        serde_json::to_string(&menu).unwrap(),
        "The menu should have Mash Airdodge"
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let inputs = args.get(1);
    let menu;
    let menu_defaults;

    unsafe {
        menu = ui_menu(MENU);
        menu_defaults = (ui_menu(MENU), serde_json::to_string(&MENU).unwrap());
    }

    #[cfg(not(feature = "has_terminal"))] {
        let (mut terminal, mut app) = test_backend_setup(menu, menu_defaults)?;
        if inputs.is_some() {
            inputs.unwrap().split(",").for_each(|input| {
                match input.to_uppercase().as_str() {
                    "X" => app.on_x(),
                    "L" => app.on_l(),
                    "R" => app.on_r(),
                    "O" => app.on_zl(),
                    "P" => app.on_zr(),
                    "A" => app.on_a(),
                    "B" => app.on_b(),
                    "UP" => app.on_up(),
                    "DOWN" => app.on_down(),
                    "LEFT" => app.on_left(),
                    "RIGHT" => app.on_right(),
                    _ => {}
                }
            })
        }
        let frame_res = terminal.draw(|f| training_mod_tui::ui(f, &mut app))?;
        let menu_json = app.get_menu_selections();

        for (i, cell) in frame_res.buffer.content().iter().enumerate() {
            print!("{}", cell.symbol);
            if i % frame_res.area.width as usize == frame_res.area.width as usize - 1 {
                println!();
            }
        }
        println!();

        println!("Menu:\n{menu_json}");
    }

    #[cfg(feature = "has_terminal")] {
        let app = training_mod_tui::App::new(menu, menu_defaults);

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
            println!("JSONs: {:#?}", res.as_ref().unwrap());
            unsafe {
                MENU = serde_json::from_str::<TrainingModpackMenu>(&res.as_ref().unwrap().0).unwrap();
                println!("MENU: {:#?}", MENU);
                let defaults_menu = serde_json::from_str::<TrainingModpackMenu>(&res.as_ref().unwrap().1).unwrap();
                println!("DEFAULTS MENU: {:#?}", defaults_menu);
            }
        }
    }

    Ok(())
}

#[cfg(feature = "has_terminal")]
fn run_app<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: training_mod_tui::App,
    tick_rate: Duration,
) -> io::Result<(String, String)> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| training_mod_tui::ui(f, &mut app).clone())?;
        let (menu_json, defaults_menu_json) = app.get_menu_selections();

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok((menu_json, defaults_menu_json)),
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
