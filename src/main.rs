mod arguments;
mod message;

use std::fs::{File};
use std::{fs, io};
use chrono::prelude::*;
use std::time::Instant;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::{BufRead, BufReader, BufWriter, Write};

fn format_filename_f(request: Vec<String>) -> String{
    let mut temp_filename = String::new();
    for r in &request{
        temp_filename.push_str(r.as_str());
        if request.len()>1||request.iter().position(|i| &i == &r).unwrap()!=request.len()-1{temp_filename.push('-');}
    }
    temp_filename
}

fn generate_filename(search_type: String, request: Vec<String>) -> String{
    let current_date = Local::today();
    let current_time = Local::now().format("%H-%M-%S").to_string();

    let formatted_date = current_date.format("%d.%m.%Y").to_string();
    let formatted_time = current_time;
    let formatted_text;
    if search_type == "-F" {
        let temp_filename = format_filename_f(request.clone());
        formatted_text = format!("{}_{}_{}", temp_filename, formatted_time, formatted_date);
    }
    else{
        let temp_filename = format_filename_f(request.clone());
        formatted_text = format!("{}_{}_{}", temp_filename, formatted_time, formatted_date);
    }
    return formatted_text;
}
fn write_full_r(path: String, mut reader: BufReader<File>, request: Vec<String>) -> Result<i32, io::Error>{
    let mut writer = BufWriter::new(File::create(path.clone()).unwrap());

    let mut lines_sorted = 0;

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if request.iter().any(|i| line.contains(i)) {
            lines_sorted += 1;
            write!(writer, "{}", line)?;
        }
        line.clear();
    }
    writer.flush()?;

    Ok(lines_sorted)
}

fn write_creds_r(path: String, mut reader: BufReader<File>, request: Vec<String>) -> Result<i32, io::Error>{
    let mut writer = BufWriter::new(File::create(path.clone()).unwrap());

    let mut lines_sorted = 0;

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if request.iter().any(|i| line.contains(i)) {
            lines_sorted += 1;
            let line_substr = line.splitn(3, ':').collect::<Vec<&str>>();
            match line_substr.len(){
                2=>writeln!(writer, "{}", line_substr[1])?,
                _=>writeln!(writer, "{}", line_substr[2])?
            }
        }
        line.clear();
    }
    writer.flush()?;

    Ok(lines_sorted)
}
fn write_full(path: String, mut reader: BufReader<File>, request: Vec<String>) -> Result<i32, io::Error>{
    let mut writer = BufWriter::new(File::create(path.clone()).unwrap());

    let mut lines_sorted = 0;

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if request.iter().any(|i| line.to_ascii_lowercase().contains(&*i.to_ascii_lowercase())) {
            lines_sorted += 1;
            write!(writer, "{}", line)?;
        }
        line.clear();
    }
    writer.flush()?;

    Ok(lines_sorted)
}

fn write_creds(path: String, mut reader: BufReader<File>, request: Vec<String>) -> Result<i32, io::Error>{
    let mut writer = BufWriter::new(File::create(path.clone()).unwrap());

    let mut lines_sorted = 0;

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if request.iter().any(|i| line.to_ascii_lowercase().contains(&*i.to_ascii_lowercase())) {
            lines_sorted += 1;
            let line_substr = line.splitn(3, ':').collect::<Vec<&str>>();
            match line_substr.len(){
                2=>writeln!(writer, "{}", line_substr[1])?,
                _=>writeln!(writer, "{}", line_substr[2])?
            }
        }
        line.clear();
    }
    writer.flush()?;

    Ok(lines_sorted)
}

fn write_file(path: String, request: Vec<String>, lines_writing: String, register: bool, search_type: String) -> Result<(), io::Error> {
    let mut paths = vec![];
    if fs::metadata(path.clone()).unwrap().is_dir(){
        for file in fs::read_dir(path.clone()).unwrap(){
            paths.push(file.unwrap().path().to_str().unwrap().to_string());
        }
    }
    else{paths.push(path.clone());}
    if !fs::metadata("result").is_ok(){fs::create_dir("result").unwrap();}
    let start = Instant::now();
    let filename = generate_filename(search_type.clone(), request.clone());
    File::create(format!("result/{}.txt", filename)).unwrap();
    message::start_message(paths.clone(), request.clone(), lines_writing.clone(), register.clone(), search_type);
    let mut lines_sorted = 0;
    for p in paths {
        let file = File::open(p)?;
        let reader = BufReader::new(file);
        //let lines = read_lines(p).expect("err").flatten();
        match register {
            true=> {
                match lines_writing.as_str() {
                    "credentials"=>{
                        lines_sorted = write_creds_r(format!("result/{}.txt", filename), reader, request.clone())?;
                    }

                    "full"=>{
                        lines_sorted = write_full_r(format!("result/{}.txt", filename), reader, request.clone())?;
                    }
                    _=>{}
                }
            }
            false=> {
                match lines_writing.as_str() {
                    "credentials" => {
                        lines_sorted = write_creds(format!("result/{}.txt", filename), reader, request.clone())?;
                    }
                    "full" => {
                        lines_sorted = write_full(format!("result/{}.txt", filename), reader, request.clone())?;
                    }
                    _ => {}
                }
            }
        }
    }
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    writeln!(&mut stdout, "====================").unwrap();
    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Green))).unwrap();
    write!(&mut stdout, "Status: Done").unwrap();
    stdout.reset().unwrap();
    let duration = start.elapsed();
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;
    write!(&mut stdout, "{}", format!("\nRequest: {:?}\n====================\nTime spent: {:02}:{:02}:{:02}\nSorted lines: ", request, hours, minutes, seconds)).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    writeln!(&mut stdout, "{}", lines_sorted).unwrap();
    stdout.reset().unwrap();
    println!();
    Ok(())
}

fn main(){
    let arguments = arguments::load_arguments();
    write_file(arguments.path, arguments.request, arguments.lines, arguments.register, arguments.request_flag).unwrap();
}