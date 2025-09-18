#[derive(Debug)]
enum MenuChoice {
    StartNewGame,
    ContinueSavedGame,
    GameplaySettings,
    QuitGame,
}

// This will bring all of the enum variants into scope so that you can
// refer to them directly.
use MenuChoice::*;

fn main() {
    //_________________________________________________________________________
    // SECTION: How to display enums

    let mut menu_selected: MenuChoice = StartNewGame;
    println!("The player selected the menu {:?}", menu_selected);
    // The player selected the menu StartNewGame

    menu_selected = ContinueSavedGame;
    println!("The player selected the menu {:?}", menu_selected);
    // The player selected the menu ContinueSavedGame

    menu_selected = GameplaySettings;
    println!("The player selected the menu {:?}", menu_selected);
    // The player selected the menu GameplaySettings

    menu_selected = QuitGame;
    println!("The player selected the menu {:?}", menu_selected);
    // The player selected the menu QuitGame
}
