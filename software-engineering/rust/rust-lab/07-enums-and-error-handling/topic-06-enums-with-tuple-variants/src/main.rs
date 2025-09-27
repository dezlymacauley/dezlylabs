enum AttackDamage {
    SwordAttack(i32),
    MagicAttack(i32),
    GunAttack(i32),
}

use AttackDamage::*;

fn main() {
    let attack_one: AttackDamage = SwordAttack(30);
    let attack_two: AttackDamage = MagicAttack(600);
    let attack_three: AttackDamage = GunAttack(150);

    match attack_one {
        SwordAttack(strength) => println!("Sword attack: {}", strength),
        MagicAttack(strength) => println!("Magic attack: {}", strength),
        GunAttack(strength) => println!("Gun attack: {}", strength),
    }
    
    match attack_two {
        SwordAttack(strength) => println!("Sword attack: {}", strength),
        MagicAttack(strength) => println!("Magic attack: {}", strength),
        GunAttack(strength) => println!("Gun attack: {}", strength),
    }

    match attack_three {
        SwordAttack(strength) => println!("Sword attack: {}", strength),
        MagicAttack(strength) => println!("Magic attack: {}", strength),
        GunAttack(strength) => println!("Gun attack: {}", strength),
    }

    // Sword attack: 30
    // Magic attack: 600
    // Gun attack: 150

}
