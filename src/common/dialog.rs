use crate::common::is_emulator;
use skyline_web::dialog::{Dialog, DialogOption};
use skyline_web::dialog_ok::DialogOk;

/// Returns true for yes, false for no
/// Returns false if you cancel with B
/// Returns `emulator_default` if you're on emulator
pub fn yes_no(prompt: String, emulator_default: bool) -> bool {
    if is_emulator() {
        return emulator_default;
    }
    Dialog::yes_no(prompt)
}

/// Returns true for yes, false for no
/// Returns false if you cancel with B
/// Returns `emulator_default` if you're on emulator
pub fn no_yes(prompt: String, emulator_default: bool) -> bool {
    if is_emulator() {
        return emulator_default;
    }
    Dialog::no_yes(prompt)
}

/// Returns true for ok, false for cancel
/// Returns false if you cancel with B
/// Returns `emulator_default` if you're on emulator
pub fn ok_cancel(prompt: String, emulator_default: bool) -> bool {
    if is_emulator() {
        return emulator_default;
    }
    Dialog::ok_cancel(prompt)
}

/// Returns `left` for the left option,
/// Returns `right` for the right option
/// Returns `default` if you cancel with B
/// Returns `emulator_default` if you're on emulator
pub fn left_right(
    prompt: String,
    left: String,
    right: String,
    default: String,
    emulator_default: String,
) -> String {
    if is_emulator() {
        return emulator_default;
    }
    match Dialog::new(prompt, left.clone(), right.clone()).show() {
        DialogOption::Left => left,
        DialogOption::Right => right,
        DialogOption::Default => default,
    }
}

/// Always returns true after you accept the prompt
pub fn dialog_ok(prompt: String) -> bool {
    if is_emulator() {
        return true;
    }
    DialogOk::ok(prompt)
}
