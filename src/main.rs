use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Print, Stylize},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::{io::stdout, vec};

use std::time::Duration;

fn main() {
    let tab: Vec<String> = vec![
        "Apple",
        "Banana",
        "Pineapple",
        "Strawberry",
        "Orange",
        "Pear",
        "Kiwi",
        "Mango",
        "Peach",
        "Cherry",
        "Apricot",
        "Grape",
        "Melon",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let mut my_list = List::create(tab);
    my_list = my_list.offset(10, 10).show();

    let _ = disable_raw_mode();
    println!(
        "Selected Item : {}. {}",
        my_list.choice, my_list.options[my_list.choice]
    );
}

struct List {
    options: Vec<String>,
    selected: i32,
    x: u16,
    y: u16,
    choice: usize,
}

impl List {
    fn create(option_tab: Vec<String>) -> Self {
        Self {
            options: option_tab,
            selected: 0,
            x: 0,
            y: 0,
            choice: 0,
        }
    }

    fn show(mut self) -> Self {
        execute!(stdout(), EnterAlternateScreen).unwrap();
        let _ = enable_raw_mode();
        execute!(stdout(), cursor::MoveTo(self.x, self.y)).unwrap();
        execute!(stdout(), cursor::Hide).unwrap();
        self.initial_print();
        self.select(0);
        loop {
            if let Ok(k) = self.keyboard_detection() {
                execute!(stdout(), LeaveAlternateScreen).unwrap();
                self.choice = k as usize;
                return self;
            }
        }
    }

    fn offset(mut self, x_offset: u16, y_offset: u16) -> Self {
        self.x = x_offset;
        self.y = y_offset;
        return self;
    }

    fn initial_print(&self) {
        for i in 0..self.options.len() {
            let line = format!("░  {}. {}", i, &self.options[i]);
            execute!(
                stdout(),
                cursor::MoveTo(self.x, self.y + i as u16),
                Print(line.white())
            )
            .unwrap();
        }
    }

    fn select(&self, pos: u16) {
        let real_y = self.y + pos;

        let line = format!("  {}. {}", pos.to_string(), &self.options[pos as usize]);
        execute!(stdout(), cursor::MoveTo(self.x, real_y)).unwrap();
        execute!(stdout(), cursor::MoveTo(self.x, real_y), Print("░".red())).unwrap();
        execute!(stdout(), Print(line.on_white().black())).unwrap();
    }

    fn unselect(&self, pos: u16) {
        let real_y = self.y + pos;

        let line = format!("░  {}. {}", pos.to_string(), &self.options[pos as usize]);
        execute!(stdout(), cursor::MoveTo(self.x, real_y)).unwrap();
        execute!(stdout(), Print(line.white())).unwrap();
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
