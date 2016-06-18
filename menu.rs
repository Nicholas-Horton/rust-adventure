// Menu for a text-based adventure game.
// (Also learn how to import/use code from another file!)

#[derive(Clone)]
struct MenuOption {
  selector: char,
  description: String,
}

pub fn display_menu() {
  let options : [MenuOption; 3] = [MenuOption { selector: 'n', description: "New Game".to_string(), },
                                   MenuOption { selector: 'l', description: "Load Game".to_string(), },
                                   MenuOption { selector: 'q', description: "Quit".to_string(), }];

  println!("Welcome to The Great Rust Adventure!");
  println!("=-----------------------------------=\n");
  println!("  Menu  ");
  println!("--------");

  for i in 0..options.len() {
    display_menu_option(options[i].clone());
  }

}

fn display_menu_option(opt:MenuOption) {
  println!(" {sel} - {desc} ", sel=opt.selector, desc=opt.description)
}
