fn main() {
    let name = String::from("Mehmet");

    println!("Character at index 8: {}", match name.chars().nth(8){
        Some(c) => c.to_string(),
        None => "No character at index 8!".to_string()
    }); //match bir option dondurecek | some ve none kullanacak aslinda

    println!("\n\n");

    println!("Occupation is {}", match get_occupation("Mehmet"){
        Some(o) => o,
        None => "No occupaiton found!"
    });
}

fn get_occupation(namee: &str) -> Option<&str> {
    match namee{
        "Mehmet" => Some("Computer Engineer"),
        "Michael" => Some("Dentist"),
        _ => None
    }
}