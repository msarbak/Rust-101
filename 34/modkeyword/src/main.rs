mod domates {
    fn chicken() {
        println!("Chicken!");
    }
    pub fn print_message(){
        chicken(); // private oldugu icin boyle kullanabiliriz
        println!("How is it going!");
    }
    pub mod water {
        pub fn print_message() {
            println!("I am water!");
        }
    }
}

fn main() {
    domates::print_message();
    domates::water::print_message();
}
