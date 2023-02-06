/*

Cargo.toml:

[dependencies]
rand = "0.3"

*/

extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1,11);  // 1 - 10 arasindaki sayialri olusturacak
    //random number generator   | gen_range(min,max);
    println!("Random number: {}", random_number);

    //flip a coin
    let random_bool = rand::thread_rng().gen_weighted_bool(25);  // 25; arttıkca true gelme olasilıgi azaliyor 2 yapilabilir
    println!("Random boolean: {}", random_bool);
}
