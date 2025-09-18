#[allow(dead_code)]
enum MenuChoice {
    StartNewGame,
    ContinueSavedGame,
    GameplaySettings,
    QuitGame,
}

use MenuChoice::*;

fn main() {

    /*

    Each variant in an enum has a discriminant value.
    The first variant in the enum has a discriminant value of 0.
    The second variant has a discriminant value of 1,
    and this pattern continues until the last variant.

    enum MenuChoice {
        StartNewGame,           // discriminant value = 0
        ContinueSavedGame,      // discriminant value = 1
        GameplaySettings,       // discriminant value = 2
        QuitGame,               // discriminant value = 3
    }

    */

    let menu_selected = QuitGame;
    println!(
        "Selected the enum variant that has a discriminant value of {}",
        menu_selected as i32
    );
    // Selected the enum variant that has a discriminant value of 3

    //_________________________________________________________________________

}


