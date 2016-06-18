// Menu for a text-based adventure game.
// (Also learn how to import/use code from another file!)

struct MenuOption {
  selector: char,
  description: String,
}

pub fn display_menu() {
  let option_new_game  : MenuOption = MenuOption { selector: 'n', description: "New Game".to_string(), };
  let option_load_game : MenuOption = MenuOption { selector: 'l', description: "Load Game".to_string(), };
  let option_quit      : MenuOption = MenuOption { selector: 'q', description: "Quit".to_string(), };

  println!("Welcome to The Great Rust Adventure!");
  println!("=-----------------------------------=\n");
  println!("  Menu  ");
  println!("--------");

  display_menu_option(option_new_game);
  display_menu_option(option_load_game);
  display_menu_option(option_quit);

}

fn display_menu_option(opt:MenuOption) {
  println!(" {sel} - {desc} ", sel=opt.selector, desc=opt.description)
}
