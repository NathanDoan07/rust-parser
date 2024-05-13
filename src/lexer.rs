/* Nathan Doan
*  Rust Programming Assignment
*  10-15-2023
*/

use regex::Regex;


//Enum for tokens the lexer can capture, also part of the grammar
#[derive(Debug, Clone)]
pub enum TokenList {
    DATA,   
    INPUT,  
    PROCESS,
    OUTPUT,
    END,
    ID(String),
    Num(String),
    TRUE,
    FALSE,
    READ,
    COLON,
    COMMA,
    PERIOD,
    LPAREN,
    RPAREN,
    ASSIGN,
    VECTOR,
    NUMBER,
    REGRESSIONA,
    REGRESSIONB,
    MEAN,
    STDDEV,
    CORRELATION,
    String(String),
}

//
pub fn tokenize(input: &str) -> Vec<TokenList> {
    let mut token_vec: Vec<TokenList> = Vec::new();
    let regex_string = r#""([^"]*)""#; //Regex expression for string, for STRING token
    //TODO, only capture lowercase, currently captures anything 
    let regex_id = r"\w+"; //Regex expression to get ID, for ID token
    let regex_num = r"\d+"; //Regex expression to get any number, for NUM token

    //Regex equation to grab tokens with characters, any words not captured in bottom tokens are IDs
    let all_regex = Regex::new(&format!(r"{}|{}|:|,|\.|\(|\)|\=|{}", regex_string, regex_id, regex_num)).unwrap(); 

    //Loop to go through file and tokenize everything
    for capture in all_regex.captures_iter(input) {
        let token = capture.get(0).unwrap().as_str();
        match token {
            "data" => token_vec.push(TokenList::DATA),
            "input" => token_vec.push(TokenList::INPUT),
            "process" => token_vec.push(TokenList::PROCESS),
            "output" => token_vec.push(TokenList::OUTPUT),
            "end" => token_vec.push(TokenList::END),
            "true" => token_vec.push(TokenList::TRUE),
            "false" => token_vec.push(TokenList::FALSE),
            "read" => token_vec.push(TokenList::READ),
            "vector" => token_vec.push(TokenList::VECTOR),
            "number" => token_vec.push(TokenList::NUMBER),
            "mean" => token_vec.push(TokenList::MEAN),
            "stddev" => token_vec.push(TokenList::STDDEV),
            "regressiona" => token_vec.push(TokenList::REGRESSIONA),
            "regressionb" => token_vec.push(TokenList::REGRESSIONB),
            "correlation" => token_vec.push(TokenList::CORRELATION),
            ":" => token_vec.push(TokenList::COLON),
            "," => token_vec.push(TokenList::COMMA),
            "." => token_vec.push(TokenList::PERIOD),
            "(" => token_vec.push(TokenList::LPAREN),
            ")" => token_vec.push(TokenList::RPAREN),
            "=" => token_vec.push(TokenList::ASSIGN),
            _ => {
                //Special Case for String
                if let Some(captures) = Regex::new(regex_string).unwrap().captures(token) {
                    let string_value = captures.get(1).unwrap().as_str();
                    token_vec.push(TokenList::String(string_value.to_string()));
                
                //Special Case for Numbers
                } else if let Some(captures) = Regex::new(regex_num).unwrap().captures(token) {
                    let num_value = captures.get(0).unwrap().as_str();
                    token_vec.push(TokenList::Num(num_value.to_string()));

                //Special case for ID if it was not under num, numbers or string
                } else if let Some(captures) = Regex::new(regex_id).unwrap().captures(token) {
                    let id_value = captures.get(0).unwrap().as_str();
                    token_vec.push(TokenList::ID(id_value.to_string()));
                }
            }
        }
    }

    //Confirms that Lexer completed 
    println!("Lexical analysis complete!");
    token_vec
}
