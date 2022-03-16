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
use tui::{
    backend::{Backend},
    widgets::ListState,
    Terminal,
};

use training_mod_consts::*;
use training_mod_tui::StatefulList;

fn main() -> Result<(), Box<dyn Error>> {
    let menu;
    unsafe {
        menu = get_menu();
    }

    let tab_specifiers = vec![
        ("Mash Settings", vec![
            "Mash Toggles",
            "Followup Toggles",
            "Attack Angle",
            "Ledge Options",
            "Ledge Delay",
            "Tech Options",
            "Miss Tech Options",
            "Defensive Options",
            "Aerial Delay",
            "OoS Offset",
            "Reaction Time",
        ]),
        ("Defensive Settings", vec![
            "Fast Fall",
            "Fast Fall Delay",
            "Falling Aerials",
            "Full Hop",
            "Shield Tilt",
            "DI Direction",
            "SDI Direction",
            "Airdodge Direction",
            "SDI Strength",
            "Shield Toggles",
            "Mirroring",
            "Throw Options",
            "Throw Delay",
            "Pummel Delay",
            "Buff Options",
        ]),
        ("Miscellaneous Settings", vec![
            "Input Delay",
            "Save States",
            "Save Damage",
            "Hitbox Visualization",
            "Stage Hazards",
            "Frame Advantage",
            "Mash In Neutral"
        ])
    ];
    let mut tabs: std::collections::HashMap<&str, Vec<SubMenu>> = std::collections::HashMap::new();
    tabs.insert("Mash Settings", vec![]);
    tabs.insert("Defensive Settings", vec![]);
    tabs.insert("Miscellaneous Settings", vec![]);

    let mut menu_items = Vec::new();
    for sub_menu in menu.sub_menus.iter() {
        for tab_spec in tab_specifiers.iter() {
            if tab_spec.1.contains(&sub_menu.title) {
                tabs.get_mut(tab_spec.0).unwrap().push(sub_menu.clone());
                menu_items.push( sub_menu.clone());
            }
        }
    };

    let app = training_mod_tui::App::new(tabs, menu_items, 3);

    #[cfg(not(feature = "has_terminal"))] {
        let backend = tui::TestBackend::new(100, 8);
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
    loop {
        terminal.draw(|f| training_mod_tui::ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        #[cfg(feature = "has_terminal")]
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
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