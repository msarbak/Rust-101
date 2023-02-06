/*
[dependencies]
regex = "0.2"
*/

extern crate regex;
use regex::Regex;

fn main() {
    let text = "domates";

    let re = Regex::new(r"\w{5}").unwrap(); 
    //kelimede 5 harf var mi    | unwrap() guvenli kullanim icin
    
    println!("Found match? {}", re.is_match(text));

    let re2 = Regex::new(r"(\w{5})").unwrap(); 

    match re.captures(text)//eslesen kismi yakalayacak
    { 
        Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),
        // caps.get(0) -> eslesen 0 noktasi
        // unwrap() ->  yakalanan yapiyi dondurur
        // as_str() ->  yakalanani string yapar

        None =>  println!("Couldn't find match!")
    }
}
