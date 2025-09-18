enum CharacterType {
    Swordsman,
    Wizard,
    Gunslinger,
}

impl CharacterType {
    fn long_distance_attack_strength(&self) -> i32 {
        let lda_strength = match self {
            CharacterType::Swordsman => 15,
            CharacterType::Wizard => 50,
            CharacterType::Gunslinger => 40,
        };

        lda_strength
    }
}

use CharacterType::*;

fn main() {
    let swordsman_one: CharacterType = Swordsman;
    let wizard_one: CharacterType = Wizard;
    let gunslinger_one: CharacterType = Gunslinger;

    println!(
        "lda_strength swordsman_one: {}",
        swordsman_one.long_distance_attack_strength()
    );
    
    println!(
        "lda_strength wizard_one: {}",
        wizard_one.long_distance_attack_strength()
    );

    println!(
        "lda_strength gunslinger_one: {}",
        gunslinger_one.long_distance_attack_strength()
    );

    // lda_strength swordsman_one: 15
    // lda_strength wizard_one: 50
    // lda_strength gunslinger_one: 40

}
