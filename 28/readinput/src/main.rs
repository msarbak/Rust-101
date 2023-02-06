
use std::io;

fn main() {
    let mut input = String::new();

    println!("Hey mate! Say something:");

    match io::stdin().read_line(&mut input){
        Ok(_) => {
            // println!("success! you said {}", input);
            println!("success! you said {}", input.to_uppercase());
        },
        Err(e) => println!("oops! something went wrong! {}", e)
    }
}
