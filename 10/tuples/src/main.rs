fn main() { // tuples -> diziler

    let tupl = (20, 25, 30, 35);
    println!("{}", tupl.2); // result: 30

    let tup2 = (20, "mehmet", 3.2, true);
    println!("{}", tup2.3); // result: true
    
    let tup3 = (20, "mehmet", 3.2, true, (1, 4, 7));
    println!("{}", (tup3.4).2); // result: 7
    //burada adım adım gidiyoruz. önce diziye erismek istedigimizi belirtiyoruz akabinde dizinin erismek istedigimiz elemanının indeksini belirtiyoruz. 

    
    let tup4 = (45, 6.7, "computer");
    let (a, b, c) = tup4; // diziyi kopyaladık
    println!("a is {}", a); // tek tek yazdirmis olduk
    println!("b is {}", b);
    println!("c is {}", c);

}
