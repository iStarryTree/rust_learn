use std::env;
use std::fs;

fn main(){
    // let args:Vec<String> = env::args().collect();
    let args = env::args().collect::<Vec<String>>();

    let query= &args[1];
    let file_path = &args[2];

    println!("Searching for {} in {}", query, file_path);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");

    print!("With text:\n{}", contents);

}