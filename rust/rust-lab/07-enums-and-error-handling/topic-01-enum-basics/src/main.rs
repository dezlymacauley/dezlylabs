/*

 ABOUT: Enums

An enum is a data type that contains a fixed list of potential values.
The enum can only be set to one of those values at a time.

The technical name for these values is `enum variants`,
and the word `enum` is short for `enumeration`.

A practical use case for an enum is when you want to represent something
that has a fixed set of states.

E.g. The menu choices of a video game.

The name of the enum and its variants must be in UpperCamelCase.

*/

#[derive(Debug)]
enum MenuChoice {
    StartNewGame,
    ContinueSavedGame,
    GameplaySettings,
    QuitGame,
}

fn main() {
    //_________________________________________________________________________
    // SECTION: How to display enums

    let mut menu_selected: MenuChoice = MenuChoice::StartNewGame;
    println!("The player selected the menu {:?}", menu_selected);
    // The player selected the menu StartNewGame

    menu_selected = MenuChoice::ContinueSavedGame;
    println!("The player selected the menu {:?}", menu_selected);
    // The player selected the menu ContinueSavedGame

    menu_selected = MenuChoice::GameplaySettings;
    println!("The player selected the menu {:?}", menu_selected);
    // The player selected the menu GameplaySettings

    menu_selected = MenuChoice::QuitGame;
    println!("The player selected the menu {:?}", menu_selected);
    // The player selected the menu QuitGame

    //_________________________________________________________________________
    // SECTION: Another way to display enums

    println!("============================================");

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

    menu_selected = MenuChoice::QuitGame;
    println!(
        "Selected the enum variant that has a discriminant value of {}",
        menu_selected as i32
    );
    // Selected the enum variant that has a discriminant value of 3

    //_________________________________________________________________________
}
