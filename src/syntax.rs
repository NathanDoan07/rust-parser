/* Nathan Doan
*  Rust Programming Assignment
*  10-15-2023
*/

use crate::lexer::TokenList;

//GENERALIZED PARSE FOR ALL MAJOR GRAMMAR FOR PROGRAM ->
/*Note: 
* https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/ 
* Big help for referencing how to structure the parser/more explanations on parsing
* (Using stack-like implementation)
*/

pub fn program_parse(token_vec: &mut Vec<TokenList>, flag: &str) -> Result<(), String> {
    println!("Parsing..");
    //Checks flag for program
    let _flag = flag;
    //Determins current_token to run through different functions (datadefs, inputops, processops, outputops)
    let mut current_token = "Data";
    while !token_vec.is_empty() {
        match token_vec.first() {
            //Checks if DATA is in the tokens, if so changes "current token" to "input" and moves into lower level datadefs grammar
            Some(&TokenList::DATA) => {
                if current_token == "Data" {
                    current_token = "Input";
                    let _ = datadefs(token_vec, _flag);
                } else {
                    return Err("Program is not in the correct grammar, did it start at data? Ending program.".to_string());
                }
            }
            //Checks if INPUT is in the tokens, if so changes "current token" to "process" and moves into lower level inputops grammar
            Some(&TokenList::INPUT) => {
                if current_token == "Input" {
                    current_token = "Process";
                    let _ = inputops(token_vec, _flag);
                } else {
                    return Err("Program is not in the correct grammar, is it data -> input? Ending Program.".to_string());
                }
            }
            //Checks if PROCESS is in the tokens, if so changes "current token" to "input" and moves into lower level processops grammar
            Some(&TokenList::PROCESS) => {
                if current_token == "Process" {
                    current_token = "Output";
                    let _ = processops(token_vec, _flag);
                } else {
                    return Err("Program is not in the correct grammar, is it input -> process? Ending Program.".to_string());
                }
            }
            //Checks if OUTPUT is in the tokens, if so it should complete if END. is the ending token, moves into lower level outputops
            Some(&TokenList::OUTPUT) => {
                if current_token == "Output" {
                    let _ = outputops(token_vec, _flag);
                } else {
                    return Err("Program is not in the correct grammar, did it end with output? Ending Program.".to_string());
                }
            }
            //Overall Error Check
            Some(unexpected_token) => {
                return Err(format!("Unepected token: {:?}", unexpected_token));
            }
            //In case nothing is sent through, the program does nothing
            None => {
              

            }
        }
    }

    //Once the entire program is run through properly it means the file was successfully parsed, prints it out and returns function as passable
    println!("Parsing Successful");

    Ok(())
}

//GRAMMAR FOR DATADEFS 

pub fn datadefs(tokens: &mut Vec<TokenList>, flag: &str) -> Result<(), String> {
    //Flag to check for outputs
    let _flag = flag;

    //Looks if the first two tokens are DATA, COLON
    if let Some(TokenList::DATA) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected DATA token".to_string());
    }

    if let Some(TokenList::COLON) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected COLON after DATA".to_string());
    }
   
    //If first two tokens pass, it moves onto ID and moves into lower level datadef grammar
    while let Some(token) = tokens.first() {
        match token {
            TokenList::ID(_) => {
                let _ = datadef(tokens, _flag);
                if let Some(TokenList::COMMA) = tokens.first() {
                    tokens.remove(0);
                } 
            }

            //Once it finds INPUT, the function breaks and moves back to the program_parse
            TokenList::INPUT => {
                break;
            }

            //General error checking
            _ => {
                return Err("Unexpected token in datadefs".to_string());
            }
        }
    }
    Ok(())
}

//LOWER LEVEL GRAMMAR FOR DATADEFS

fn datadef(tokens: &mut Vec<TokenList>, _flag: &str) -> Result<(), String> {

    //Program overall checks if it follows grammar of ID, COLON, TYPE[VECTOR, NUMBER], COMMA
    if let Some(TokenList::ID(_)) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected ID".to_string());
    }

    if let Some(TokenList::COLON) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected Colon".to_string());
    }

    if let Some(TokenList::VECTOR) = tokens.first() {
        tokens.remove(0);
    } else if let Some(TokenList::NUMBER) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected Vector or Number".to_string());
    }

    //Checks if it ends with a comma, if it does, moves back into the function
    //If it ends with INPUT the function passes and moves back into program_parse
    if let Some(TokenList::COMMA) = tokens.first() {
        tokens.remove(0);
    } else if let Some(TokenList::INPUT) = tokens.first() {
        return Ok(());
    } else {
        return Err("Expected Comma or INPUT".to_string());
    }

    Ok(())
}

//GRAMMAR FOR INPUTOPS

pub fn inputops(tokens: &mut Vec<TokenList>, flag: &str) -> Result<(), String> {
    //Flags for output
    let _flag = flag;

    //Looks if the first two tokens are INPUT, COLON
    if let Some(TokenList::INPUT) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected Input token".to_string());
    }

    if let Some(TokenList::COLON) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected colon after input".to_string());
    }

    //If first two tokens pass, it moves onto ID and moves into lower level inputops grammar
    while let Some(token) = tokens.first() {
        match token {
            TokenList::ID(_) => {
                let _ = inputop(tokens, _flag);
                if let Some(TokenList::ASSIGN) = tokens.first() {
                    tokens.remove(0);
                } 
            }

            //Once it finds PROCESS, the function breaks and moves back to the program_parse
            TokenList::PROCESS => {
                break;
            }

            //General error checking
            _ => {
                return Err("Unexpected token in inputops".to_string());
            }
        }
    }
    Ok(())
}

//LOWER LEVEL GRAMMAR FOR INPUTOPS

fn inputop(tokens: &mut Vec<TokenList>, flag: &str) -> Result<(), String> {
    //Flag to check for outputs
    let _flag = flag;

    //Value storage for outputs for scheme and prolog, mainly used for prolog
    let mut id_value = String::new();
    let mut bool_value = String::new();
    let mut num_value = String::new();
    let mut string_value = String::new();


    /*Program overall checks if it follows grammar of 
    * ID, ASSIGN, READ, LPAREN, String, COMMA, TRUE/FALSE, COMMA, NUM, RPAREN, COMMA
    * Below ID, TRUE/FALSE, NUM, STRING are flag checks for which output to process
    */
    if let Some(TokenList::ID(id)) = tokens.first() {
        if flag == "-s" {
            print!("(define {}", id);
        } else {
            println!("main :-");
            id_value = id.clone();
        }
        tokens.remove(0);
    } else {
        return Err("Expected ID".to_string());
    }

    if let Some(TokenList::ASSIGN) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected Assign".to_string());
    }

    if let Some(TokenList::READ) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected Read".to_string());
    }

    if let Some(TokenList::LPAREN) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected Left Paren".to_string());
    }

    if let Some(TokenList::String(string)) = tokens.first() {
        if flag == "-s" {
            print!("(read-csv \"./{}\"", string);
        } else {
            string_value = string.clone();
        }
        tokens.remove(0);
    } else {
        return Err("Expected string".to_string());
    }

    if let Some(TokenList::COMMA) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected comma".to_string());
    }

    if let Some(TokenList::TRUE) = tokens.first() {
        if flag == "-s" {
            print!(" #t");
        } else {
            bool_value = "true".to_string();
        }
        tokens.remove(0);
    } else if let Some(TokenList::FALSE) = tokens.first() {
        if flag == "-s" {
            print!(" #f");
        } else {
            bool_value = "false".to_string();
        }
        tokens.remove(0);
    } else {
        return Err("Expected true/false (bool)".to_string());
    }

    if let Some(TokenList::COMMA) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected comma".to_string());
    }

    if let Some(TokenList::Num(num)) = tokens.first() {
        if flag == "-s" {
            println!(" {}))", num);
        } else {
            num_value = num.clone();
        }
        tokens.remove(0);
    } else {
        return Err("Expected num(0-9)".to_string());
    }

    if let Some(TokenList::RPAREN) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected right paren".to_string());
    }
    
    //Flag for Prolog Output
    if flag == "-p" {
        println!("load_data_column('{}', {}, {}, {}),", string_value, bool_value, num_value, id_value);
    }

    //Checks if it ends with a comma, if it does, moves back itno the function
    //If it ends with PROCESS the function passes and moves back into program_parse 
    if let Some(TokenList::COMMA) = tokens.first() {
        tokens.remove(0);
    } else if let Some(TokenList::PROCESS) = tokens.first() {
        return Ok(());
    } else {
        return Err("Expected Comma or Process".to_string());
    }
    Ok(())
}

//GRAMMAR FOR PROCESSOPS

pub fn processops(tokens: &mut Vec<TokenList>, flag: &str) -> Result<(), String> {
    //Flag for output
    let _flag = flag;

    //Looks if the first two tokens are PROCESS, COLON
    if let Some(TokenList::PROCESS) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected PROCESS token".to_string());
    }

    if let Some(TokenList::COLON) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected COLON after PROCESS".to_string());
    }

    //If first two tokens pass, it moves onto ID and moves into lower level processops grammar
    while let Some(token) = tokens.first() {
        match token {
            TokenList::ID(_) => {
                let _ = processop(tokens, _flag);
                if let Some(TokenList::ASSIGN) = tokens.first() {
                    tokens.remove(0);
                }
            }

            //Once it finds OUTPUT, the function breaks and moves back to the program_parse
            TokenList::OUTPUT => {
                break;
            }
            _ => {
                return Err("Unexpected token in processops".to_string());
            }
        }
    }
    Ok(())
}

//LOWER LEVEL GRAMMAR FOR PROCESSOPS

fn processop(tokens: &mut Vec<TokenList>, flag: &str) -> Result<(), String> {
    //Flag to check for outputs
    let _flag = flag;
    let mut id_value = String::new();

    /*Program overall checks if it follows grammar of 
    * ID, ASSIGN, (Regressiona/b, Correlation), LPAREN, ID, COMMA, ID, RPAREN, COMMA
    * OR ID, ASSIGN, (Mean, Stddev), LPAREN, ID, RPAREN, COMMA
    * Below ID and Regression/Mean Branches are flag checked for which output to process
    */
    if let Some(TokenList::ID(id)) = tokens.first() {
        if flag == "-s" {
            print!("(define {} ", id.clone());
        } else {
            id_value = id.clone();
        }
        tokens.remove(0);
    } else {
        return Err("Expected ID".to_string());
    }

    if let Some(TokenList::ASSIGN) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected Assign".to_string());
    }

    //All branches of PROCESSOP -->
    if let Some(
        TokenList::REGRESSIONA | TokenList::REGRESSIONB | 
        TokenList::MEAN | TokenList::STDDEV | TokenList::CORRELATION) = tokens.first() {
        
        //Checking Regression/Correlation tokens
        if let Some(TokenList::REGRESSIONA |TokenList::REGRESSIONB | TokenList::CORRELATION) = tokens.first() {
            let process_op = match tokens.remove(0) {
                TokenList::REGRESSIONA => "regressiona",
                TokenList::REGRESSIONB => "regressionb",
                TokenList::CORRELATION => "correlation",
                _ => {
                    return Err("Expected regression/correlation".to_string());
                }
            };
            if flag == "-s" {
                print!("({}", process_op);
            } else {
                print!("{}", process_op);
            }
           
            if let Some(TokenList::LPAREN) = tokens.first() {
                tokens.remove(0);
            } else {
                return Err("Expected Left Paren".to_string());
            }
        
            if let Some(TokenList::ID(id)) = tokens.first() {
                if flag == "-s" {
                    print!(" {}", id);
                } else {
                    print!("({}, ", id);
                }
                tokens.remove(0);
            } else {
                return Err("Expected ID".to_string());
            }
        
            if let Some(TokenList::COMMA) = tokens.first() {
                tokens.remove(0);
            } else {
                return Err("Expected comma".to_string());
            }
        
            if let Some(TokenList::ID(id)) = tokens.first() {
                if flag == "-s" {
                    println!(" {}))", id);
                } else {
                    print!("{},", id);
                }
                tokens.remove(0);
            } else {
                return Err("Expected ID".to_string());
            }
        
            if let Some(TokenList::RPAREN) = tokens.first() {
                tokens.remove(0);
            } else {
                return Err("Expected right paren".to_string());
            }

            //MEAN AND STDDEV CHECK -> BRANCH BREAKS OFF
        } else if let Some(TokenList::MEAN | TokenList::STDDEV) = tokens.first() {
            let process_op = match tokens.remove(0) {
                TokenList::MEAN => "mean",
                TokenList::STDDEV => "stddev",
                _ => {
                    return Err("Expected mean/stddev".to_string());
                }
            };
            if flag == "-s" {
                print!("({}", process_op);
            } else {
                print!("{}(", process_op);
            }

            if let Some(TokenList::LPAREN) = tokens.first() {
                tokens.remove(0);
            } else {
                return Err("Expected Left Paren".to_string());
            }
        
            if let Some(TokenList::ID(id)) = tokens.first() {
                if flag == "-s" {
                    println!(" {}))", id);
                } else {
                    print!("{},", id);
                }
                tokens.remove(0);
            } else {
                return Err("Expected ID".to_string());
            }

            if let Some(TokenList::RPAREN) = tokens.first() {
                tokens.remove(0);
            } else {
                return Err("Expected right paren".to_string());
            }

        } else {
            return Err("Checking between branches broke".to_string());
        } 
    } else {
        return Err("Expected Regressions, Mean, Stddev, Correlation tokens".to_string());
    }

    //Flag check for prolog output
    if flag == "-p" {
        println!(" {})),", id_value);
    }

    //Checks if it ends with a comma, if it does, move back into the function
    //If it ends with OUTPUT the function passes and moves back into program_parse
    if let Some(TokenList::COMMA) = tokens.first() {
        tokens.remove(0);
    } else if let Some(TokenList::OUTPUT) = tokens.first() {
        return Ok(());
    } else {
        return Err("Expected comma or output".to_string())
    }
    Ok(())
} 

//GRAMMAR FOR OUTPUTOPS

pub fn outputops(tokens: &mut Vec<TokenList>, flag: &str) -> Result<(), String> {
    //Flag to check for outputs
    let _flag = flag;

    //Looks if the first two tokens are OUTPUT, COLON
    if let Some(TokenList::OUTPUT) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected Output token".to_string());
    }

    if let Some(TokenList::COLON) = tokens.first() {
        tokens.remove(0);
    } else {
        return Err("Expected colon after input".to_string());
    }

    //If first ttwo tokens pass, it moves onto string and id into lower level outputop grammar
    while let Some(token) = tokens.first() {
        match token {
            TokenList::String(_) => {
                let _ = outputop(tokens, _flag);
                if let Some(TokenList::COMMA) = tokens.first() {
                    tokens.remove(0);
                }
            }
            TokenList::ID(_) => {
                let _ = outputop(tokens, _flag);
                if let Some(TokenList::COMMA) = tokens.first() {
                    tokens.remove(0);
                }
            }

            //General error check
            _ => {
                return Err("Unexpecetd token in outputops".to_string());
            }
        }
    }
    Ok(())
}

//LOWER LEVEL GRAMMAR FOR OUTPUTOP

fn outputop(tokens: &mut Vec<TokenList>, flag: &str) -> Result<(), String> {
    //Flag to check for outputs
    let _flag = flag;

    /* Program overall check if it follows grammar of
    *  ID | STRING, COMMA
    *  Below ID and STRING are output flag checks for the output
    */
    if let Some(TokenList::ID(_) | TokenList::String(_)) = tokens.first() {
        match tokens.first() {
            Some(TokenList::ID(id)) => {
                if flag == "-s" {
                    println!("(display {})", id);
                    println!("(newline)");
                } else {
                    println!("writeln({})", id);
                }
            }
            Some(TokenList::String(string)) => {
                if flag == "-s" {
                    println!("(display \"{}\")", string);
                    println!("(newline)");
                } else {
                    println!("writeln(\"{}\")", string);
                }
            }
            _ => {
                return Err("Expected id or string".to_string());
            }
        };
        tokens.remove(0);
    } else {
        return Err("Expected ID or string".to_string());
    }

    if let Some(TokenList::COMMA) = tokens.first() {
        tokens.remove(0);
    } 
    
    //Checking final [END, PERIOD] tokens
    else if let Some(TokenList::END) = tokens.first() {
        tokens.remove(0);

        if let Some(TokenList::PERIOD) = tokens.first() {
            tokens.remove(0);
        } else {
            return Err("Expected period".to_string());
        }
    } else {
        return Err("Expected comma or (End -> Period)".to_string());
    }
    Ok(())
}