mod app;
mod submenu;
mod tab;
mod toggle;
pub use app::*;
pub use submenu::*;
pub use tab::*;
pub use toggle::*;

pub trait InputControl {
    fn on_a(&mut self);
    fn on_b(&mut self);
    fn on_x(&mut self);
    fn on_y(&mut self);
    fn on_up(&mut self);
    fn on_down(&mut self);
    fn on_left(&mut self);
    fn on_right(&mut self);
    fn on_start(&mut self);
    fn on_l(&mut self);
    fn on_r(&mut self);
    fn on_zl(&mut self);
    fn on_zr(&mut self);
}
