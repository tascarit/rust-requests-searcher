use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::arguments;
use std::io::Write;

pub fn error_message(arguments: arguments::Arguments, msg: String){
    println!("{}", format!("Lines: {}\nPath: {}\nRegister: {}\nSearch Type: {}", arguments.lines, arguments.path, if arguments.register {"true"} else {"false"}, if arguments.request_flag=="-F" {"plain text"} else {"regex"}));
    println!("====================");
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Red))).unwrap();
    write!(&mut stdout,"{}", msg).unwrap();
    stdout.reset().unwrap();
    println!("\n====================");
}

pub fn start_message(paths: Vec<String>, request: Vec<String>, lines: String, register: bool, search_type: String){
    println!("{}", format!("Lines: {}\nRequest: {:?}\nPath: {:?}\nRegister: {}\nSearch type: {}", lines, request, paths, register, if search_type=="-F" {"plain text"} else if search_type=="-R" {"regex"} else {""}));
}