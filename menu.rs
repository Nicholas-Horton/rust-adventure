// Menu for a text-based adventure game.
// (Also learn how to import/use code from another file!)

use std::io::{self, BufRead};

#[derive(Clone)]
struct MenuOption {
  selector: char,
  description: &'static str,
}

static OPTIONS : [MenuOption; 3] = [MenuOption { selector: 'n', description: "New Game", },
                                    MenuOption { selector: 'l', description: "Load Game", },
                                    MenuOption { selector: 'q', description: "Quit", }];

pub fn display_menu() {
  let mut has_selected_option: bool = false;

  while !has_selected_option {
    println!("  Menu  ");
    println!("--------");
    for i in 0..OPTIONS.len() {
      display_menu_option(OPTIONS[i].clone());
  }
    has_selected_option = read_menu_option();
  }
}

fn display_menu_option(opt:MenuOption) {
  println!(" {sel} - {desc} ", sel=opt.selector, desc=opt.description)
}

fn read_menu_option() -> bool {
  let mut opt = String::new();
  let stdin = io::stdin();
  stdin.lock().read_line(&mut opt).unwrap();

  for i in 0..OPTIONS.len() {
    if opt == OPTIONS[i].clone().selector.to_string() {
      return true;
    }
  }

  return false;
}
