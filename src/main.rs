const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT:i32 = 2;

fn main() {

    let (mut missle, mut ready) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} of my {} missles", missle, ready);
    println!("Missles left: {}", missle - ready);
}
