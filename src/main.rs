use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, Stylize},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::{io::Write, time::Duration};
use std::{
    io::{Read, stdout},
    vec,
};

// | ‡ | # ░ + ▌▐ ▓ ┇ ║ ┃
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
        "Watermelon",
        "Plum",
        "Blueberry",
        "Raspberry",
        "Blackberry",
        "Papaya",
        "Lemon",
        "Lime",
        "Pomegranate",
        "Coconut",
        "Fig",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let mut my_list = List::create(tab);
    my_list = my_list
        .offset(20, 5)
        .numbered(true)
        .sbg(Color::White)
        .stxt(Color::Black)
        .scroll_bar('▌', Color::Red)
        .show();

    if my_list.choice != -1 {
        println!(
            "Selected Item : {}. {}",
            my_list.choice, my_list.options[my_list.choice as usize]
        );
    }

    print!("Press Enter to Exit");
    stdout().flush().unwrap();
    let _ = std::io::stdin().read(&mut [0u8]);
}

struct List {
    options: Vec<String>,
    selected: i32,
    x: u16,
    y: u16,
    choice: i32,
    scroll_bar_icon: char,
    numbered_list: bool,
    selected_bg: Color,
    selected_txt: Color,
    selected_icon: Color,
    top_item: i32,
    max_item: u16,
}

impl List {
    fn create(option_tab: Vec<String>) -> Self {
        Self {
            options: option_tab,
            selected: 0,
            x: 0,
            y: 0,
            choice: 0,
            scroll_bar_icon: '▌',
            numbered_list: true,
            selected_bg: Color::White,
            selected_txt: Color::Black,
            selected_icon: Color::Red,
            top_item: 0,
            max_item: 10,
        }
    }

    fn offset(mut self, x_offset: u16, y_offset: u16) -> Self {
        self.x = x_offset;
        self.y = y_offset;
        self
    }

    fn scroll_bar(mut self, icon: char, color: Color) -> Self {
        self.selected_icon = color;
        self.scroll_bar_icon = icon;
        self
    }

    fn numbered(mut self, choice: bool) -> Self {
        self.numbered_list = choice;
        self
    }

    fn sbg(mut self, color: Color) -> Self {
        self.selected_bg = color;
        return self;
    }

    fn stxt(mut self, color: Color) -> Self {
        self.selected_txt = color;
        return self;
    }

    fn show(mut self) -> Self {
        execute!(stdout(), EnterAlternateScreen).unwrap();
        let _ = enable_raw_mode();
        execute!(stdout(), cursor::MoveTo(self.x, self.y)).unwrap();
        execute!(stdout(), cursor::Hide).unwrap();
        self.initial_print();
        self.select(0);
        loop {
            let keyboard_result = self.keyboard_detection();
            if let Ok(k) = keyboard_result {
                self.leave();
                self.choice = k;
                return self;
            } else if keyboard_result == Err(true) {
                self.leave();
                self.choice = -1;
                return self;
            }
        }
    }

    fn leave(&self) {
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        let _ = disable_raw_mode();
    }

    fn initial_print(&self) {
        for i in 0..self.options.len() {
            let real_i = i + self.top_item as usize;
            let line = if self.numbered_list {
                format!(
                    "{} {:>3}. {}",
                    self.scroll_bar_icon, real_i, &self.options[real_i]
                )
            } else {
                format!("{} {}", self.scroll_bar_icon, &self.options[real_i])
            };

            execute!(
                stdout(),
                cursor::MoveTo(self.x, self.y + real_i as u16),
                Print(line.white())
            )
            .unwrap();
        }
    }

    fn select(&self, pos: u16) {
        let real_y = self.y + pos;

        let line = if self.numbered_list {
            format!("{:>3}. {}", pos, &self.options[pos as usize])
        } else {
            format!("{}", &self.options[pos as usize])
        };

        // Printing the scroll bar icon
        let icon: String = format!("{} ", self.scroll_bar_icon);
        execute!(
            stdout(),
            cursor::MoveTo(self.x, real_y),
            Print(icon.with(self.selected_icon))
        )
        .unwrap();

        // Printing the styled line
        let styled_line = line.with(self.selected_txt).on(self.selected_bg);
        execute!(stdout(), Print(styled_line)).unwrap();
    }

    fn unselect(&self, pos: u16) {
        let real_y = self.y + pos;

        let line = if self.numbered_list {
            format!(
                "{} {:>3}. {}",
                self.scroll_bar_icon, pos, &self.options[pos as usize]
            )
        } else {
            format!("{} {}", self.scroll_bar_icon, &self.options[pos as usize])
        };
        execute!(stdout(), cursor::MoveTo(self.x, real_y)).unwrap();
        execute!(stdout(), Print(line.white())).unwrap();
    }

    fn keyboard_detection(&mut self) -> Result<i32, bool> {
        let old_selected = self.selected;
        let mut new_selected: i32 = 99;
        let k = key_pressed();
        match k {
            -99 => return Err(true),
            -66 => return Ok(old_selected),
            -100 => (),
            _ => new_selected = self.selected + k as i32,
        }

        if new_selected < 0 || new_selected >= self.options.len() as i32 {
            return Err(false);
        } else if old_selected != new_selected {
            self.selected = new_selected;
            self.unselect(old_selected as u16);
            self.select(self.selected as u16);
        }
        Err(false)
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
