enum AttackDamage {
    SwordAttack { damage: i32 },
    MagicAttack { damage: i32, mana_cost: i32 },
    GunAttack { damage: i32, ammo_cost: i32 },
}

use AttackDamage::*;

fn main() {
    let attack_one = SwordAttack { damage: 30 };
    let attack_two = MagicAttack { damage: 600, mana_cost: 50 };
    let attack_three = GunAttack { damage: 150, ammo_cost: 1 };

    match attack_one {
        SwordAttack { damage } => println!("Sword attack: {}", damage),

        MagicAttack { damage, mana_cost } => {
            println!("Magic attack: {}", damage);
            println!("Mana cost: {}", mana_cost);
        },
        GunAttack { damage, ammo_cost } => {
            println!("Gun attack: {}", damage);
            println!("Ammo cost: {}", ammo_cost);
        },
    }

    match attack_two {
        SwordAttack { damage } => println!("Sword attack: {}", damage),

        MagicAttack { damage, mana_cost } => {
            println!("Magic attack: {}", damage);
            println!("Mana cost: {}", mana_cost);
        },
        GunAttack { damage, ammo_cost } => {
            println!();
            println!("Gun attack: {}", damage);
            println!("Ammo cost: {}", ammo_cost);
        },
    }

    match attack_three {
        SwordAttack { damage } => println!("Sword attack: {}", damage),

        MagicAttack { damage, mana_cost } => {
            println!("Magic attack: {}", damage);
            println!("Mana cost: {}", mana_cost);
        },
        GunAttack { damage, ammo_cost } => {
            println!("Gun attack: {}", damage);
            println!("Ammo cost: {}", ammo_cost);
        },
    }

    // Sword attack: 30
    // Magic attack: 600
    // Mana cost: 50
    // Gun attack: 150
    // Ammo cost: 1
}
