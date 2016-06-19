// Text-based adventure game for learning rust!

mod menu;

fn main() {
  println!("Welcome to The Great Rust Adventure!");
  println!("=-----------------------------------=\n");
  menu::display_menu();
}
