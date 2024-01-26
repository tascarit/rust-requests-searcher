use std::env;
use std::process::exit;
use crate::message;
pub struct Arguments{
    pub path: String,
    pub lines: String,
    pub request: Vec<String>,
    pub register: bool,
    pub request_flag: String
}

pub fn load_arguments() -> Arguments{
    let args: Vec<String> = env::args().collect();

    if args.len() < 4{
        let arguments = processing_flag_console(args);
        message::error_message(arguments, String::from("Status: ERROR - Invalid Arguments"));
        exit(0);
    }
    else{
        let mut arguments = processing_flag_console(args.clone());
        if arguments.lines != "credentials"{
            arguments.lines = String::from("full");
        }
        if !file_exists_check(arguments.path.clone()){
            message::error_message(arguments, String::from("Status: ERROR - File or Directory does not exist")) ;
            exit(0);
        }
        return arguments;
    }

}

fn file_exists_check(path: String) -> bool{
    if !std::fs::metadata(path).is_ok(){ return false; }
    true
}

fn request_common(args: Vec<String>, i: usize, mut arguments: Arguments) -> Arguments{
    if (i+1)<args.len(){
        let mut j = 1;
        while ((j+i) < args.len()) && (!args[j+i].contains('-')){
            arguments.request.push(args[i+j].clone());
            j+=1;
        }
        arguments.request_flag = String::from("-F");
    }
    arguments
}

fn request_regex(args: Vec<String>, i: usize, mut arguments: Arguments) -> Arguments{
    if (i+1)<args.len(){
        let mut j = 1;
        let mut s = String::new();
        while ((j+i) < args.len()) && (!args[j+i].contains('-')){
            s+=args[j+i].clone().as_str();
            s+=" ";
            j+=1;
        };
        s.pop();
        arguments.request.push(s);
        arguments.request_flag = String::from("-R");
    }
    arguments
}

fn processing_flag_console(args: Vec<String>) -> Arguments{
    let mut arguments = Arguments{
        path:String::from(""),
        lines:String::from(""),
        request:vec![],
        register:false,
        request_flag:String::from("")
    };
    for i in 0..args.len(){
        match args[i].as_str(){
            "-P"=>{
                if (i+1)<args.len(){
                    arguments.path = args[i+1].clone();
                }
                continue;
            },
            "-C"=>{
                arguments.lines = String::from("credentials");
                continue;
            },
            "-r"=>{
                arguments.register=true;
                continue;
            },
            "-F1"=>{
                arguments = request_common(args.clone(), i, arguments);
                continue;
            }
            "-F2"=>{
                arguments = request_common(args.clone(), i, arguments);
                continue;
            }
            "-F3"=>{
                arguments = request_common(args.clone(), i, arguments);
                continue;
            }
            "-F4"=>{
                arguments = request_common(args.clone(), i, arguments);
                continue;
            }
            "-F5"=>{
                arguments = request_common(args.clone(), i, arguments);
                continue;
            }
            "-R1"=>{
                arguments = request_regex(args.clone(), i, arguments);
                continue;
            }
            "-R2"=>{
                arguments = request_regex(args.clone(), i, arguments);
                continue;
            }
            "-R3"=>{
                arguments = request_regex(args.clone(), i, arguments);
                continue;
            }
            "-R4"=>{
                arguments = request_regex(args.clone(), i, arguments);
                continue;
            }
            "-R5"=>{
                arguments = request_regex(args.clone(), i, arguments);
                continue;
            }
            "-R"=>{
                if arguments.request.len() == 0 {
                    arguments = request_regex(args.clone(), i, arguments);
                    continue;
                }
                else{continue;}
            }
            "-F"=>{
                if arguments.request.len() == 0 {
                    arguments = request_common(args.clone(), i, arguments);
                    continue;
                }
                else{continue;}
            }

            _=>{continue;}
        }
    }

    arguments
}