fn main() {
    // Replace
    {
        let my_string = String::from("Rust is fantastic!"); 
        println!("After Replace: {}", my_string.replace("fantastic", "great"));
    }

    // Lines , line formatting
    {
        let my_string = String::from("The weather is\nnice\noutside mate!");
        for line in my_string.lines(){
            println!("[ {} ]", line);
        }
    }

    // Split
    {
        let my_string = String::from("Leave+a+coffee+please");
        let tokens: Vec<&str> = my_string.split("+").collect(); 

        println!("{}", my_string);
        println!("At index 2: {}", tokens[2]); //coffee
    }

    // Trim
    {
        let my_string = String::from("   My name is Mehmet  \n\r");

        println!("Before trim {}", my_string);
        println!("After trim {}", my_string.trim()); //boslukları temizledi
    }

    //Chars
    {
        let my_string = String::from("Mehmet bilgisayar basinda");
        println!("{}", my_string);

        //my_string[4];
        //get character at index
        match my_string.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("no character at index 4.")
        }
    }
}
