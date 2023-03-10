struct Rectangle {
    width: u8,
    height: u8
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height // kareyse true dondurur
    }
}

fn main() {
    
}

fn give_two() -> i32 {
    2
}

// run commend: cargo test
#[cfg(test)]
mod mehmet_tests{

    #[test]
    #[should_panic]
    fn test_basic() {
        assert!(1 == 1); // test pass!
        panic!("oh no!");
    }

    #[test]
    fn test_equals() {
        assert_eq!(super::give_two(), 1 + 1); // 1 + 1 = 2 equal
        assert_ne!(super::give_two(), 1 + 2); // not equal
    }

    #[test]
    #[ignore]
    fn test_equalss() {
        assert_eq!(2, 1 + 1); // 1 + 1 = 2 equal
        assert_ne!(2, 1 + 2); // not equal
    }

    #[test]
    #[should_panic]
    fn test_structs() {
        let r =  super::Rectangle {
            width: 50,
            height: 25
        };

        assert!(r.is_square());
    }
}