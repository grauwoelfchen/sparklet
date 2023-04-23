use pancurses::{endwin, initscr};

fn main() {
    let window = initscr();
    window.printw("Sparklet TUI 0.1.0");
    window.refresh();
    window.getch();
    endwin();
}
