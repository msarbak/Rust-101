use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    //Add values
    marks.insert("Rust", 67);
    marks.insert("C#", 43);
    marks.insert("Java", 12);
    marks.insert("HTML", 89);
    //Two parameters: key, value

    //find length of HashMap:
    println!("how many subjects  have you studied? {}", marks.len()); // 4

    //get a single value:
    match marks.get("Java"){
        Some(mark) => println!("you got {} for Java!", mark),
        None => println!("you didn't study Java!")
    }

    //remove a value:
    marks.remove("C#");

    //loop
    //for (key, value)
    for (subject, mark) in &marks {
        println!("for {} you got {}%", subject, mark);
    }

    //check for value:
    println!("did you study C++ ? {}", marks.contains_key("C++")); //false
    println!("did you study HTML ? {}", marks.contains_key("HTML")); //true

}
