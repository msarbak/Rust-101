/*
[dependencies]
reqwest = "0.8.3"
*/

extern crate reqwest;

fn main() {
   //1
    let response_text = reqwest::get("http://youtube.local/hello").expect("couldn't make request").text().expect("couldn't read response text");
    println!("response text: {}", response_text);

   //2
   /*
    match reqwest::get("http://youtube.local/hello"){
        Ok(mut response) => {
            //check if 200 OK
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(Test) => println!("Response Text: {}", text),
                    Err(_) => println!("coundn't response text")
                }
            } else {
                println!("Response was not 200 OK.");
            }
        }
        Err(_) => println!("Couldn't make the request!")
    }*/
}
