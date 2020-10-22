use zoo_debugger::process;
use zoo_debugger::debug_service;
use zoo_debugger::error;
use std::error::Error;
use core::ffi::c_void;

#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgGroup};

fn parse_args() -> (Option<String>, Option<Vec<String>>, Option<i32>) {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("new")
            .help("new target program")
            .short("n")
            .long("new")
            .takes_value(true)
        )
        .arg(Arg::with_name("args")
            .help("args of new target program")
            .short("a")
            .long("args")
            .takes_value(true)
            .multiple(true)
        )
        .arg(Arg::with_name("process")
            .help("attach process")
            .short("p")
            .long("process")
            .takes_value(true)
        )
        .group(ArgGroup::with_name("n_or_p").args(&["new", "process"]).required(true)
        );
            
    let matches = app.get_matches();
    let mut prog: Option<String> = None;
    if matches.is_present("new") {
        prog = Some(matches.values_of("new").unwrap().collect());
    }
    let mut args: Option<Vec<String>> = None;
    if matches.is_present("args") {
        args = Some(matches.values_of("args").unwrap().
            map(|x|x.to_string()).collect());
    }
    let mut pid: Option<i32> = None;
    if matches.is_present("process") {
        let t: String = matches.values_of("process").unwrap().collect();
        pid = Some(t.parse().unwrap());
    }
    (prog, args, pid)
}

fn run() -> Result<(), error::AllError> {
    let (prog, args, pid) = parse_args();

    let mut o: debug_service::DebugService<process::UnixProcess>; 
    match prog {
        Some(progn) => {
            o = debug_service::DebugService::new_and_attach(&progn, args)?;
        },
        None => {o = debug_service::DebugService::attach(pid.unwrap())?;},
    }

    loop {
        let (command, value) = read_line();
        match &command as &str {
            "b" | "break" => {
                if value.is_none() {
                    println!("Please input address.");
                    continue;
                }
                o.insert_break(value.unwrap())?;
            }, 
            "c" | "cont" => o.cont()?,
            "i" | "info" => {
                let info = o.get_info()?;
                println!("{}", info);
            },
            "q" | "quit" => {
                o.detach()?;
                break;
            },
            _ => println!("unknown command."),
        }
    }
    Ok(())
}

fn read_line() -> (String, Option<*mut c_void>) {
    print!("Input: ");
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut s: Vec<String> = s.trim().split_whitespace().map(|x|x.to_string()).collect();
    let command = std::mem::replace(&mut s[0], "dummy".to_string());
    if s.len() == 1 {
        (command, None) 
    } else {
        (command, Some(read_hex(&s[1])))
    }
}

fn read_hex(s: &str) -> *mut c_void {
    i64::from_str_radix(&s[2..s.len()], 16).unwrap() as *mut c_void
}

fn print_trace<E: Error>(e: E) {
    eprintln!("{}", e);
    let mut source = e.source();
    while let Some(e) = source {
        eprintln!("{}", e);
        source = e.source()
    }
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => print_trace(e),
    }
}

