fn main() {
    //print_numbers_to(10);
    print_numbers_to(20);
    
    /*if is_even(30){
        println!("it is even..!");
    }*/
}

fn print_numbers_to(num: u32){ // num: parametre, u32: 32 bit integer turu
    for n in 1..num {
        //println!("{}", n);
        if is_even(n) {
            println!("{} is even!", n);
        } else {
            println!("{} is odd", n);
        }
    } 
}

fn is_even(num: u32) -> bool { //bool: return type, num: paramter, u32: parameter type
    return num % 2 == 0; //sayi ciftse true doner, degilse false
}