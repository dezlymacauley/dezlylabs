#[allow(dead_code)]
enum MenuChoice {
    StartNewGame = 10,
    ContinueSavedGame = 5,
    GameplaySettings = 20,
    QuitGame = 4,
}

use MenuChoice::*;

fn main() {

    /*

    enum MenuChoice {
        StartNewGame,           // index 10
        ContinueSavedGame,      // index 5
        GameplaySettings,       // index 20
        QuitGame,               // index 4
    }

    */

    let menu_selected = GameplaySettings;
    println!(
        "Selected the enum variant with the discriminant value of {}",
        menu_selected as i32
    );
    // Selected the enum variant with the discriminant value of 20

    //_________________________________________________________________________
}
