
use hokm::gui::gui_main;

pub fn main() {
    if let Err(e) = gui_main() {
        println!("Error: {}", e);
    }
}
