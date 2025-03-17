
pub use yew::prelude::*;
pub use stylist::{css, style, Style, yew::Global, StyleSource};
pub use super::style::*;
pub use std::f64;

#[derive(Clone)]
pub struct Calculation {
    pub display: f64,
    pub pre_num: f64,
    pub is_selected: bool,
    pub select_mode: f64,
}
