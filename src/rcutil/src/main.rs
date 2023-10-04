git//#use clap::{Arg, App};
// use clap::Parser;


extern crate clap;
use std::path::PathBuf;
use clap::{arg, command, value_parser, ArgAction, Arg, Command};

use clap::Parser;



    
// #[derive(Parser, Debug)]
// #[commmand(author, version,  name = "rsutil", about = "Rust system utility refer: pyutil")]
// struct Args {
//     #[arg(short, long)]
//     name: String,

//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
// }

fn build_command() {
    let matches = command!()
        .arg(arg!([name] "optional name to operate on"))
        .arg(
            arg!(
                -c  --config <FIEL> "Set a custome config file."
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -d --debug ... "Turn debugging info on."
            )
        )
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(
                    -l --list "list test values"
                ).action(ArgAction::SetTrue)),
        )
        .get_matches();

    if let Some(name) = matches.get_one::<String>("name") {
        println!("name: {}", name);
    }

    if let Some(config) = matches.get_one::<PathBuf>("config") {
        println!("config: {:?}", config);
    }

    match matches 
        .get_one::<u8>("debug")
        .expect("Count's are defauled") {
            0 => println!("Debugging is off"),
            1 => println!("Debugging is kind of on"),
            2 => println!("Debugging is on"),
            _ => println!("Don't be crazy"),
        }
    
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.get_flag("list") {
            println!("listing test values");
        } else {
            println!("not listing test values");
        }
    }

}

fn parse_args() -> clap::ArgMatches  {
    let matches = command!()
        //.arg(arg!([name] "optional name to operate on"))
        .version("0.1.0")
        .author("Bruce Zhang Song")
        .about("Rust system utility refer: pyutil")
        .arg(
            arg!(
                -y  --config <FIEL> "Set a custome config file."
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -v --verbose "Sets the level of verbosity"
            )
            .required(false)
        )   
        .arg(
            arg!(
                -c --command <CMD> "Sets command to execute"
            )
            
        )
        .get_matches();


    return matches

    // if let Some(name) = matches.get_one::<String>("name") {
    //     println!("name: {}", name);
    // } else {
    //     println!("name: None");
    // }

}

fn dispatch_command(matches: clap::ArgMatches) {
    if let Some(config) = matches.get_one::<PathBuf>("config") {
        println!("config: {:?}", config);
    } else {
        println!("config: None");
    }

    

    if let Some(verbose) = matches.get_one::<bool>("verbose") {
        println!("verbose: {}", verbose);
    } else {
        println!("verbose: None");
    }

    let ls_command_str = String::from("ls");//.to_string();
    let command_str =matches.get_one::<String>("command").expect("command is required");//.to_string();
    println!("command: {}", command_str);
    let s = String::from("hello");
    match &command_str as &str {
            "ls" => println!(" matched with: ls command"),
          
            //String("ps") => println!("ps command"),
            //String("df") => println!("df command"),
            _ => println!("Don't be crazy provide a valid command"),
        }
}

fn main() {

    let arg_matches = parse_args();
    dispatch_command(arg_matches);
    //build_command();
    
    

    // let yaml_config = matches.value_of("yaml_config")
    //                         .unwrap_or("rsutil.yaml");
    // println!("Value for yaml_config: {}", yaml_config);

    // let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello, {}!", args.name);
    // }

    
}
