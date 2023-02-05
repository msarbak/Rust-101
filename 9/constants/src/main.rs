//constants globalde cagrilmalı

const MAXIMUM_NUMBER: u8 = 20; //unsigned 8 bit integer

fn main() {
    for n in 1..MAXIMUM_NUMBER {
        println!("{}", n);
    }

    //MAXIMUM_NUMBER = 30; 
    //error -> constant values are not changable!
}
