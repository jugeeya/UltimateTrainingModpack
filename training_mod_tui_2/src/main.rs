use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::{
    io,
    time::{Duration, Instant},
};

use training_mod_tui_2::{
    App, AppPage, InputControl, StatefulList, StatefulSlider, StatefulTable, SubMenu, SubMenuType,
    Tab, Toggle, NX_SUBMENU_COLUMNS, NX_SUBMENU_ROWS,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = create_app();
    let json = "{\"Menu Open Start Press\":[1,0],\"Dmg Range (CPU)\":[40,100]}";
    app.update_from_json(json);
    let mut terminal = setup_terminal()?;

    let tick_rate = Duration::from_millis(250);
    let res = run_app(&mut terminal, app, tick_rate);
    restore_terminal(terminal)?;

    if let Err(err) = res {
        println!("Error: {:?}", err)
    } else {
        println!("JSON: {:#?}", res.as_ref().unwrap());
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn create_app<'a>() -> App<'a> {
    let mut app = App::new();
    let mut button_tab_submenus: Vec<SubMenu> = Vec::new();
    button_tab_submenus.push(SubMenu {
        title: "Menu Open Start Press",
        id: "menu_open_start_press",
        help_text: "Menu Open Start Press: Should pressing start open the menu?",
        submenu_type: SubMenuType::ToggleSingle,
        toggles: new_toggle_table(new_on_off()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Save State Save",
        id: "save_state_save",
        help_text: "Save State Save: Hold any one button and press the others to trigger",
        submenu_type: SubMenuType::ToggleMultiple,
        toggles: new_toggle_table(new_button_combo()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Save State Load",
        id: "save_state_load",
        help_text: "Save State Load: Hold any one button and press the others to trigger",
        submenu_type: SubMenuType::ToggleMultiple,
        toggles: new_toggle_table(new_button_combo()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Input Record",
        id: "input_record",
        help_text: "Input Record: Hold any one button and press the others to trigger",
        submenu_type: SubMenuType::ToggleMultiple,
        toggles: new_toggle_table(new_button_combo()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Input Playback",
        id: "input_playback",
        help_text: "Input Playback: Hold any one button and press the others to trigger",
        submenu_type: SubMenuType::ToggleMultiple,
        toggles: new_toggle_table(new_button_combo()),
        slider: None,
    });

    let button_tab = Tab {
        id: "button",
        title: "Button Config",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            button_tab_submenus.clone(),
        ),
    };
    let button_tab_2 = Tab {
        id: "button",
        title: "Button Config 2",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            button_tab_submenus.clone(),
        ),
    };
    let button_tab_3 = Tab {
        id: "button",
        title: "Button Config 3",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            button_tab_submenus.clone(),
        ),
    };
    let button_tab_4 = Tab {
        id: "button",
        title: "Button Config 4",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            button_tab_submenus.clone(),
        ),
    };
    let button_tab_5 = Tab {
        id: "button",
        title: "Button Config 5",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            button_tab_submenus.clone(),
        ),
    };

    let mut save_state_tab_submenus: Vec<SubMenu> = Vec::new();
    save_state_tab_submenus.push(SubMenu {
        title: "Mirroring",
        id: "save_state_mirroring",
        help_text:
            "Mirroring: Flips save states in the left-right direction across the stage center",
        submenu_type: SubMenuType::ToggleSingle,
        toggles: new_toggle_table(new_on_off()),
        slider: None,
    });
    save_state_tab_submenus.push(SubMenu {
        title: "Auto Save States",
        id: "save_state_autoload",
        help_text: "Auto Save States: Load save state when any fighter dies",
        submenu_type: SubMenuType::ToggleSingle,
        toggles: new_toggle_table(new_on_off()),
        slider: None,
    });
    save_state_tab_submenus.push(SubMenu {
        title: "Dmg Range (CPU)",
        id: "save_damage_limits_cpu",
        help_text: "Limits on random damage to apply to the CPU when loading a save state",
        submenu_type: SubMenuType::Slider,
        toggles: StatefulTable::new(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS),
        slider: Some(StatefulSlider::new()),
    });
    let save_states_tab = Tab {
        id: "save_state",
        title: "Save States",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            save_state_tab_submenus,
        ),
    };

    app.tabs = StatefulList::with_items(vec![
        button_tab,
        button_tab_2,
        button_tab_3,
        button_tab_4,
        button_tab_5,
        save_states_tab,
    ]);
    app
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: training_mod_tui_2::App,
    tick_rate: Duration,
) -> io::Result<String> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| training_mod_tui_2::render_ui(f, &mut app))?;
        if app.page == AppPage::CLOSE {
            return Ok(app.to_json());
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.page = AppPage::CLOSE,
                    KeyCode::Char('a') => app.on_a(),
                    KeyCode::Char('b') => app.on_b(),
                    KeyCode::Char('x') => app.on_x(),
                    KeyCode::Char('y') => app.on_y(),
                    KeyCode::Char('o') => app.on_zl(),
                    KeyCode::Char('p') => app.on_zr(),
                    KeyCode::Char('l') => app.on_l(),
                    KeyCode::Char('r') => app.on_r(),
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

fn new_button_combo<'a>() -> Vec<Toggle<'a>> {
    let a_button = Toggle {
        title: "A Button",
        value: 0,
        max: 1,
    };
    let b_button = Toggle {
        title: "B Button",
        value: 0,
        max: 1,
    };
    let x_button = Toggle {
        title: "X Button",
        value: 0,
        max: 1,
    };
    let y_button = Toggle {
        title: "Y Button",
        value: 0,
        max: 1,
    };
    let l_button = Toggle {
        title: "L Button",
        value: 0,
        max: 1,
    };
    let r_button = Toggle {
        title: "R Button",
        value: 0,
        max: 1,
    };
    let zl_button = Toggle {
        title: "ZL Button",
        value: 0,
        max: 1,
    };
    let zr_button = Toggle {
        title: "ZR Button",
        value: 0,
        max: 1,
    };
    let dpad_up_button = Toggle {
        title: "Dpad Up Button",
        value: 0,
        max: 1,
    };
    let dpad_down_button = Toggle {
        title: "Dpad Down Button",
        value: 0,
        max: 1,
    };
    let dpad_left_button = Toggle {
        title: "Dpad Left Button",
        value: 0,
        max: 1,
    };
    let dpad_right_button = Toggle {
        title: "Dpad Right Button",
        value: 0,
        max: 1,
    };
    vec![
        a_button,
        b_button,
        x_button,
        y_button,
        l_button,
        r_button,
        zl_button,
        zr_button,
        dpad_up_button,
        dpad_down_button,
        dpad_left_button,
        dpad_right_button,
    ]
}

fn new_on_off<'a>() -> Vec<Toggle<'a>> {
    let true_toggle = Toggle {
        title: "True",
        value: 0,
        max: 1,
    };
    let false_toggle = Toggle {
        title: "False",
        value: 0,
        max: 1,
    };
    vec![true_toggle, false_toggle]
}

fn new_toggle_table<'a>(items: Vec<Toggle<'a>>) -> StatefulTable<Toggle<'a>> {
    StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, items)
}
