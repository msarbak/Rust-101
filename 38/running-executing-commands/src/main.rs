use std::process::Command;

fn main() {
    //python domates.py 
    let mut cmd = Command::new("python");
    cmd.arg("domates.py");
 
    //Execute the commend
    match cmd.output() {
        Ok(o) => { // python daki print std output tan geliyor 
            unsafe {
                println!("output: {}", String::from_utf8_unchecked(o.stdout));
            }
            //from_utf8_unchecked -> bu method guvenli degil, vectoru kontrol etmiyor
        } 
        Err(e) => {
            println!("there was a error! {}", e);
        }
    }
}
