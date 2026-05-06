# CLIM

CLIM is a project developed by [Maloll](https://discordapp.com/users/970348301448806430). Its core objective is to experiment with and build various CLI navigation systems and interactive menus across different programming languages as I learn them.

While this project starts as a personal laboratory, the long-term goal is to package these systems into professional, reusable **libraries** for everyone ❤️.

## 🎯 Project Roadmap
I am currently focusing on the Rust implementation.

### Current Progress
- ✅ **Raw Mode & Alternate Screen**: Full control over the terminal buffer without polluting command history.
- ✅ **Keyboard Capture**: Real-time detection of `Up`, `Down`, `Enter`, and `Esc` keys.
- ✅ **Dynamic Selection**: Interactive visual feedback when navigating through list items.
- ✅ **Clean UI**: Automatic cursor hiding and screen cleanup on exit.

### Future Goals
- ❌ **Scrolling Support**: Implementation of "Viewports" to handle lists longer than the terminal height.
- ❌ **Search & Filtering**: Real-time string matching to find items instantly in large datasets.
- ❌ **Diverse Menu Layouts**: Beyond simple lists implementing grids, tree views, detailed view with horizontal navigation.
- ❌ **Deep Customization**: Full control over the UI allowing users to tweak colors, borders, symbols, and highlighting styles to suit any terminal theme.
- ❌ **Library Packaging**: Refactoring the code into a standalone Rust Crate.
- ❌ **Multiple Languages**: Developing equivalent versions in **Python** and **Bash**.

## 🚀 Quick Start (Rust)

The current implementation allows you to turn a simple `Vec<String>` into a fully interactive menu that returns the selected index.
```rust
let items = vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string()];
let mut menu = List::create(items);

// This will launch the interactive TUI and return the chosen index
let choice = menu.launch(); 

println!("User selected index: {}", choice);
