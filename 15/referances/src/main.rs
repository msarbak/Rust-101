fn main() {
    let mut x = 10;
    let xr = &x;

    println!("x is {}", x);
    println!("x is {}", xr);

    let dom = &mut x; // artık dom ile x i degisterebilecegiz
    *dom += 1;
    //sorun cıkarsa burayı block icine al parantezle bazen x den yana borrow oluyor

    println!("x is {}", dom);

    println!("x is {}", x);
}
//REFERANCES AND BORROWING