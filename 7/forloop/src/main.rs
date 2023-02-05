fn main() {
    //println!("Hello, world!");
    let  numbers = 30..51; // 30 dan 51 e kadar 51 dahil degil

    /*for i in 1..11 { // 1 dan 11 e kadar 51 dahil degil
        println!("the number is {}", i);
    }*/

    for i in numbers { // 1 dan 11 e kadar 51 dahil degil
        println!("the number is {}", i);
    }

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for a in animals.iter() { //iter() kullanmazsak secili elemana ulasamıyoruz
        println!("The animal name is {}", a);
    }

    for (index, a) in animals.iter().enumerate() {  //index kullanarak for dongusu
        println!("The index is {} and  the animal name is {}", index , a);
    }
}
