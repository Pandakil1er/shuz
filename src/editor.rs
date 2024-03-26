use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("failed to initialize terminal"),
            cursor_position: Position { x: 0, y: 0 },
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        if let Key::Ctrl('q') = pressed_key {
            self.should_quit = true;
        }
        Ok(())
    }
    fn draw_welcome_msg(&self) {
        let mut welcome_msg = format!("Shuz editor Version : {VERSION}\r");
        let width = self.terminal.size().width as usize;
        let len = welcome_msg.len();
        let padding = width.saturating_sub(len) / 2;
        let space = " ".repeat(padding.saturating_sub(1));
        welcome_msg = format!("~{space}{welcome_msg}");
        welcome_msg.truncate(width);
        println!("{welcome_msg}\r");
    }
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_msg();
            } else {
                println!("~\r");
            }
        }
    }
}
pub fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{e}");
}
