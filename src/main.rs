/* Nathan Doan
*  Rust Programming Assignment
*  10-15-2023
*/

//Imports for lexer/syntax
mod lexer;
mod syntax;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    //String to capture command line argument
    let args: Vec<String> = env::args().collect();

    //checks command line if its a valid argument
    if args.len() != 3 {
        eprintln!("Cargo run error, please use 'cargo run -s or -p'");
        std::process::exit(1);
    }
    //Grabs the filename and flag
    let filename = &args[1];
    let flag = &args[2];


    //Checks to make sure flags are -s and -p
    if flag == "-s" || flag =="-p" {
        println!("Processing file {}", filename);
        let _file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error opening the file: {}", err);
                std::process::exit(1);
            }
        };
       
        //Error checking for file and viable contents
        let mut file = File::open(filename)
            .expect("Should have been able to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Should have been able to read the file");
        
        
        
        //Function for the lexer to tokenize the file contents
        let mut token_vec = lexer::tokenize(&contents);

        //Runs through the file syntaxer and determines if its an error or not
        match syntax::program_parse(&mut token_vec, flag) {
            Ok(()) => {
                
            }
            Err(error) => {
                println!("Parsing error: {}", error);
            }
        }

        } else {
            println!("Please choose between -s or -p flags");
        }
}
