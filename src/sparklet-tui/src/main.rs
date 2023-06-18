use std::env;
use std::process::exit;

use pancurses::{
    chtype, curs_set, endwin, has_colors, initscr, init_color, init_pair,
    start_color, Window, COLOR_BLACK, COLOR_CYAN, COLOR_PAIR,
};

const DEFAULT_CONFIG_DIR: &str = "~/.config/sparklet";
const PAIR_INDEX_CYAN: i16 = 1;

fn main() {
    let mut win = initscr();

    if !has_colors() {
        endwin();
        eprintln!("Your terminal does not support color");
        exit(1);
    }

    curs_set(0); // off
    render(&mut win);

    win.getch();
    endwin();
}

fn get_version() -> String {
    // get these values at compile time
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    format!("{} v{}", pkg, ver)
}

fn get_config_dir() -> String {
    env::var("SPARKLET_CONFIG_DIR").unwrap_or(DEFAULT_CONFIG_DIR.to_string())
}

// renders the inital screen with version texts.
fn render(win: &mut Window) {
    init_color(0, 0, 0, 0);
    start_color();

    init_pair(PAIR_INDEX_CYAN, COLOR_CYAN, COLOR_BLACK);
    let pair = COLOR_PAIR(PAIR_INDEX_CYAN as chtype);

    win.attron(pair);

    let (rows, cols) = win.get_max_yx();
    let start_y = rows / 2;

    // sparklet-tui
    let msg = get_version();
    print_msg_in_middle(win, start_y, 0, cols, &msg);
    // sparklet
    let msg = sparklet::get_version();
    print_msg_in_middle(win, start_y + 1, 0, cols, &msg);

    // TODO: cofnig (for now, only showing config-dir set via an env var)
    let dir = get_config_dir();
    let msg = format!("config-dir: {}", dir);
    print_msg_in_middle(win, start_y + 2, 0, cols, &msg);

    win.attroff(pair);
}

// prints a message at the middle center.
fn print_msg_in_middle(
    win: &mut Window,
    start_y: i32,
    start_x: i32,
    width: i32,
    msg: &str,
) {
    let mut w = width;
    let (mut y, mut x) = (0, 0);

    if start_y != 0 {
        y = start_y;
    }
    if start_x != 0 {
        x = start_x;
    }
    if width == 0 {
        w = 80;
    }

    let len = msg.len();
    let tmp = (w - len as i32) / 2;
    x += tmp;

    win.mvprintw(y, x, msg);
    win.refresh();
}

/// An utility macro for HashMap definition.
///
/// NOTE: https://github.com/rust-lang/rust/issues/97030
///
/// ## Examples
///
/// ```rust
/// # use std::collections::HashMap;
/// #
/// # [macro_use] extern crate;
/// #
/// # fn main() {
/// #
/// let map: HashMap<&str, Option<&str>> = hmap! {
///   "Foo" => Some("Bar"),
///   "Baz" => None,
/// };
/// assert_eq!(map.get("Foo").unwrap(), Some("Bar"));
/// assert_eq!(map.get("Baz").unwrap(), None);
/// assert_eq!(map.get("Quux"), None);
/// #
/// #   Ok(())
/// # }
/// ```
#[allow(unused_macros)]
macro_rules! hmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(m.insert($key, $value);)+
            m
        }
    }
);

#[cfg(test)]
mod test {
    use super::*;

    use std::env;
    use std::collections::HashMap;
    use std::panic::{catch_unwind, resume_unwind, RefUnwindSafe, UnwindSafe};
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    type EnvVars<'a> = Vec<(&'a str, Result<String, env::VarError>)>;

    fn reset(vars: EnvVars) {
        for (k, v) in vars {
            match v {
                Ok(new_v) => env::set_var(k, new_v),
                Err(_) => env::remove_var(k),
            }
        }
    }

    // A test runner supports replaced environment variables
    fn with_env<'a, T>(map: HashMap<&'a str, Option<&str>>, test: T)
    where
        T: FnOnce() + UnwindSafe + RefUnwindSafe,
    {
        let lock = ENV_LOCK.lock().unwrap();
        let mut vars: EnvVars<'a> = Vec::new(); // original

        for (k, v) in map {
            let old_v = env::var(k);
            vars.push((k, old_v));
            match v {
                Some(v) => env::set_var(k, v),
                None => env::remove_var(k),
            }
        }
        match catch_unwind(|| {
            test();
        }) {
            Ok(_) => reset(vars),
            Err(err) => {
                reset(vars);

                drop(lock);
                resume_unwind(err);
            }
        }
    }

    #[test]
    fn test_get_config_dir() {
        with_env(HashMap::new(), || {
            let msg = get_config_dir();
            assert_eq!(msg, DEFAULT_CONFIG_DIR);
        });

        with_env(
            hmap! {
                "SPARKLET_CONFIG_DIR" => None
            },
            || {
                let msg = get_config_dir();
                assert_eq!(msg, DEFAULT_CONFIG_DIR);
            },
        );

        with_env(
            hmap! {
                "SPARKLET_CONFIG_DIR" => Some("/tmp")
            },
            || {
                let msg = get_config_dir();
                assert_eq!(msg, "/tmp");
            },
        );
    }

    #[test]
    fn test_get_version() {
        let msg = get_version();
        assert_eq!(msg, "sparklet-tui v0.1.0");
    }
}
