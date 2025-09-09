fn main() {
    let player_health: i32 = 100;

    if player_health == 100 {
        println!("Your health is full.");
    } else if player_health >= 50 {
        println!("You have taken some damage.");
    } else {
        println!("Be careful! You have lost half your health!");
    }
}
