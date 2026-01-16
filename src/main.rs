mod cli;
mod gui;

fn main() {
    println!("Do you want to play CLI gomoku (1) or GUI gomoku (2)?");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");  

    match choice.trim() {
        "1" => {
            cli::cli_gomoku::start_game();
        }
        "2" => {
            gui::gui_gomoku::start_game();
        }
        _ => {
            println!("Invalid choice. Please enter 1 or 2.");
        }
    }
}