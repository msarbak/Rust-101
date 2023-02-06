fn main() {
    let number = 15;
    let name = "Mehmet";

    match number {
        1 => println!("it is one!"),
        //2 => println!("there is two of them!"),
        2 ..= 20 => println!("it is greater than one!"),
        //for döngüsündeyken ..
        // aralık belirtirken ..=

        //10 | 11 => println!("it is either 10 or 11"),
        _ => println!("it doesn't match!")
    }

    println!("\n");

    match name {
        "Michelle" => println!("nice name, mate!"),
        "Mehmet" => println!("heey, welcome man!"),
        _ => println!("don't know your name!")
    }
}
