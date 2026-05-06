use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{self, Color, Print, Stylize},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{self, Cursor, Write, stdout},
    process::exit,
    thread::current,
    vec,
};

use std::time::Duration;

fn main() {
    let tab: Vec<String> = vec![
        "Pomme".to_string(),
        "Banane".to_string(),
        "Ananas".to_string(),
    ];
    let mut my_menu = Menu::create(tab);

    execute!(stdout(), EnterAlternateScreen).unwrap();
    let _ = enable_raw_mode();
    execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout(), cursor::Hide);
    my_menu.show();
    my_menu.select(0);
    loop {
        my_menu.Move();
    }
}

struct Menu {
    options: Vec<String>,
    selected: i32,
    x: u16,
    y: u16,
}

impl Menu {
    fn create(option_tab: Vec<String>) -> Self {
        Self {
            options: option_tab,
            selected: 0,
            x: 0,
            y: 0,
        }
    }

    fn show(&self) {
        for (i, opt) in self.options.iter().enumerate() {
            let line = i.to_string() + ". " + opt;
            println!("{}", line.on_black());
        }
    }

    fn select(&self, pos: u16) {
        let line = pos.to_string() + ". " + &self.options[pos as usize];
        execute!(stdout(), cursor::MoveTo(0, pos as u16)).unwrap();
        println!("{}", line.on_white().black());
    }

    fn unselect(&self, pos: u16) {
        let line = pos.to_string() + ". " + &self.options[pos as usize];
        execute!(stdout(), cursor::MoveTo(0, pos as u16)).unwrap();
        println!("{}", line.on_black().white());
    }

    fn Move(&mut self) {
        let old_selected = self.selected;
        let mut new_selected: i32 = 99;
        let k = key_pressed();
        match k {
            -99 => exit(100),
            -100 => (),
            _ => new_selected = self.selected + k as i32,
        }

        if new_selected < 0 || new_selected >= self.options.len() as i32 {
            return;
        } else if old_selected != new_selected {
            self.selected = new_selected;
            self.unselect(old_selected as u16);
            self.select(self.selected as u16);
        }
    }
}

fn key_pressed() -> i32 {
    if event::poll(Duration::from_millis(10)).unwrap_or(false) {
        if let Ok(Event::Key(key_pressed)) = event::read() {
            if key_pressed.kind == KeyEventKind::Press {
                return match key_pressed.code {
                    KeyCode::Up => -1,
                    KeyCode::Down => 1,
                    KeyCode::Esc => -99,
                    _ => -100,
                };
            }
        }
    }
    -100
}
