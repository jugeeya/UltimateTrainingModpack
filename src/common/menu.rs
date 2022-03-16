use crate::common::*;
use crate::events::{Event, EVENT_QUEUE};
use crate::training::frame_counter;
use crate::common::consts::get_menu_from_url;
use ramhorns::Template;
use skyline::info::get_program_id;
use skyline_web::{Background, BootDisplay, Webpage};
use smash::lib::lua_const::*;
use std::fs;
use std::path::Path;
use strum::IntoEnumIterator;
use crate::mkdir;

static mut FRAME_COUNTER_INDEX: usize = 0;
const MENU_LOCKOUT_FRAMES: u32 = 15;

pub fn init() {
    unsafe {
        FRAME_COUNTER_INDEX = frame_counter::register_counter();
        write_menu();
    }
}

pub unsafe fn menu_condition(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    // Only check for button combination if the counter is 0 (not locked out)
    match frame_counter::get_frame_count(FRAME_COUNTER_INDEX) {
        0 => {
            ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                && ControlModule::check_button_on_trriger(
                    module_accessor,
                    *CONTROL_PAD_BUTTON_APPEAL_HI,
                )
        }
        1..MENU_LOCKOUT_FRAMES => false,
        _ => {
            // Waited longer than the lockout time, reset the counter so the menu can be opened again
            frame_counter::full_reset(FRAME_COUNTER_INDEX);
            false
        }
    }
}

pub unsafe fn write_menu() {
    let tpl = Template::new(include_str!("../templates/menu.html")).unwrap();

    let overall_menu = get_menu();

    let data = tpl.render(&overall_menu);

    // Now that we have the html, write it to file
    // From skyline-web
    let program_id = get_program_id();
    let htdocs_dir = "training_modpack";
    let menu_dir_path = Path::new("sd:/atmosphere/contents")
        .join(&format!("{:016X}", program_id))
        .join(&format!("manual_html/html-document/{}.htdocs/", htdocs_dir));

    let menu_html_path = menu_dir_path
        .join("training_menu.html");

    mkdir(menu_dir_path.to_str().unwrap().as_bytes().as_ptr(), 777);
    let write_resp = fs::write(menu_html_path, data);
    if write_resp.is_err() {
        println!("Error!: {}", write_resp.err().unwrap());
    }
}

const MENU_CONF_PATH: &str = "sd:/TrainingModpack/training_modpack_menu.conf";

pub fn spawn_menu() {
    unsafe {
        frame_counter::reset_frame_count(FRAME_COUNTER_INDEX);
        frame_counter::start_counting(FRAME_COUNTER_INDEX);
    }

    let fname = "training_menu.html";
    let params = unsafe { MENU.to_url_params() };
    let page_response = Webpage::new()
        .background(Background::BlurredScreenshot)
        .htdocs_dir("training_modpack")
        .boot_display(BootDisplay::BlurredScreenshot)
        .boot_icon(true)
        .start_page(&format!("{}{}", fname, params))
        .open()
        .unwrap();

    let orig_last_url = page_response.get_last_url().unwrap();
    let last_url = &orig_last_url.replace("&save_defaults=1", "");
    unsafe {
        MENU = get_menu_from_url(MENU, last_url);
    }
    if last_url.len() != orig_last_url.len() {
        // Save as default
        unsafe {
            DEFAULT_MENU = get_menu_from_url(DEFAULT_MENU, last_url);
            write_menu();
        }
        let menu_defaults_conf_path = "sd:/TrainingModpack/training_modpack_menu_defaults.conf";
        std::fs::write(menu_defaults_conf_path, last_url)
            .expect("Failed to write default menu conf file");
    }

    std::fs::write(MENU_CONF_PATH, last_url).expect("Failed to write menu conf file");
    unsafe {
        EVENT_QUEUE.push(Event::menu_open(last_url.to_string()));
    }
}
