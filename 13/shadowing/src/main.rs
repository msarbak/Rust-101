fn main() {
    let mut x = 10;
    {
        //x = 15;
        // burada 15 olarak calısır ama sonrasını etkilemez

        let x = 15; // artık bundan sonra 15 olarak devam edecektir
    }

    println!("x is {}", x);

    let x = "x is a string";
    println!("x is {}", x);

    let x = true;
    println!("x is {}", x);
}
