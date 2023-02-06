fn main() {
    let mut my_string = String::from("Hello, i am Mehmet.");

    //length
    println!("Length: {}", my_string.len());

    //println!("\n");

    //is Empty?
    println!("String is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace(){
        println!("{}", token);
    }

    println!("Does the string contain 'Mehmet'? {}", my_string.contains("mehmet"));

    my_string.push_str(" Say hello to strings!");
    println!("{}", my_string);

}
