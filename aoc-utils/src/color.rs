pub const RESET: &str =         "\x1b[0m";

pub const BLACK: &str          = "\x1b[0;30m";
pub const RED: &str            = "\x1b[0;31m";
pub const GREEN: &str          = "\x1b[0;32m";
pub const BROWN: &str          = "\x1b[0;33m";
pub const ORANGE: &str         = "\x1b[0;33m";
pub const BLUE: &str           = "\x1b[0;34m";
pub const PURPLE: &str         = "\x1b[0;35m";
pub const CYAN: &str           = "\x1b[0;36m";
pub const LIGHT_GRAY: &str     = "\x1b[0;37m";
pub const LIGHT_GREY: &str     = "\x1b[0;37m";
pub const DARK_GRAY: &str      = "\x1b[1;30m";
pub const DARK_GREY: &str      = "\x1b[1;30m";
pub const LIGHT_RED: &str      = "\x1b[1;31m";
pub const LIGHT_GREEN: &str    = "\x1b[1;32m";
pub const YELLOW: &str         = "\x1b[1;33m";
pub const LIGHT_BLUE: &str     = "\x1b[1;34m";
pub const LIGHT_PURPLE: &str   = "\x1b[1;35m";
pub const LIGHT_CYAN: &str     = "\x1b[1;36m";
pub const WHITE: &str          = "\x1b[1;37m";

pub fn goto(x: usize, y: usize) -> String {
    format!("\x1b[{y};{x}H]")
}
