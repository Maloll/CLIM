use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Print, Stylize},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode},
};
use std::{io::stdout, vec};

use std::time::Duration;

fn main() {
    let tab: Vec<String> = vec![
        "Pomme", "Banane", "Ananas", "Fraise", "Orange", "Poire", "Kiwi", "Mangue", "Pêche",
        "Cerise", "Abricot", "Raisin", "Melon",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let mut my_list = List::create(tab);
    my_list.launch();
}

struct List {
    options: Vec<String>,
    selected: i32,
}

impl List {
    fn create(option_tab: Vec<String>) -> Self {
        Self {
            options: option_tab,
            selected: 0,
        }
    }

    fn launch(&mut self) {
        execute!(stdout(), EnterAlternateScreen).unwrap();
        let _ = enable_raw_mode();
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        execute!(stdout(), cursor::Hide).unwrap();
        self.show();
        self.select(0);
        loop {
            if let Ok(k) = self.keyboard_detection() {
                execute!(stdout(), LeaveAlternateScreen).unwrap();
                println!("Item choisi : {}. {}", k, self.options[k as usize]);
                break;
            }
        }
    }

    fn show(&self) {
        for i in 0..self.options.len() {
            let line = format!("░  {}. {}", i, &self.options[i]);
            println!("{}", line.white());
        }
    }

    fn select(&self, pos: u16) {
        let line = format!("  {}. {}", pos.to_string(), &self.options[pos as usize]);
        execute!(stdout(), cursor::MoveTo(0, pos as u16)).unwrap();
        execute!(stdout(), cursor::MoveTo(0, pos as u16), Print("░".red())).unwrap();
        println!("{}", line.on_white().black());
    }

    fn unselect(&self, pos: u16) {
        let line = format!("░  {}. {}", pos.to_string(), &self.options[pos as usize]);
        execute!(stdout(), cursor::MoveTo(0, pos as u16)).unwrap();
        println!("{}", line.white());
    }

    fn keyboard_detection(&mut self) -> Result<i32, bool> {
        let old_selected = self.selected;
        let mut new_selected: i32 = 99;
        let k = key_pressed();
        match k {
            -99 => execute!(stdout(), LeaveAlternateScreen).unwrap(),
            -100 => (),
            -66 => return Ok(old_selected),
            _ => new_selected = self.selected + k as i32,
        }

        if new_selected < 0 || new_selected >= self.options.len() as i32 {
            return Err(false);
        } else if old_selected != new_selected {
            self.selected = new_selected;
            self.unselect(old_selected as u16);
            self.select(self.selected as u16);
        }
        return Err(false);
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
                    KeyCode::Enter => -66,
                    _ => -100,
                };
            }
        }
    }
    -100
}
