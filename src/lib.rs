#[macro_use]
extern crate log;

pub mod application;
pub mod daemon;

mod choose_color;
mod color;
mod color_circle;
mod color_wheel;
mod deref_cell;
mod keyboard_backlight_widget;
mod keyboard_color_button;
mod keymap;

use crate::{
    choose_color::*, color::*, color_circle::*, daemon::*, deref_cell::*, keyboard_color_button::*,
    keymap::*,
};
pub use keyboard_backlight_widget::keyboard_backlight_widget;
