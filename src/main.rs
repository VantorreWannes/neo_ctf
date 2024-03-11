use crate::script::run;

pub mod script;

fn main() {
    println!("Hello, world!");
    //write_response_to_file().expect("I am a good programmer");
    run().expect("I am a good programmer");
}


