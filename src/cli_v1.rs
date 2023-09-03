use std::env;
use std::io::Write;
use std::process::exit;

#[deprecated]
pub struct Cli {}

impl Cli {
    pub fn help(command: Option<String>) {
        if command.is_none() {
        
        println!(
        "{}",
"usage: minimum [options] <command> [args]

options:
  -h, --help: print help info
  -v, --version: print version info

commands:
  build: compile the app
  help: print help info
  init: init a new app"
        );
        
        } else {
            let command_name = command.unwrap_or_default();
            let info: &str;
        
            match command_name.as_str() {
                "build" => info =
"usage: minimum build [options]

options:
  -m, --mode: used as the build mode [default: 'dev', option: 'preview', 'release']
        ",
            "init" => info = 
        "usage: minimum init <language> [options]
        
        args:
          <language>: use a supported language to develop
          
        options:
          -n, --name: used as the app name",
                &_ => {
                    println!("error: unknown command.");
                    exit(1);
                }
            }
        
        println!("{}", info);
        }
        exit(0);
    }
    pub fn init() -> std::io::Result<()> {
        let args_iter = Vec::from_iter(env::args());
        let args = args_iter.split_first().unwrap().1.to_vec();
        let mut options: Vec<String> = vec![];
    
        let mut index = 0;
        for input in args {
            if input == "init" {
                options = args_iter.split_at(index + 1).1.to_vec();
                break;
            }
            index += 1;
        }
    
        let options_clone = options.clone();
        let current_dir = env::current_dir()?;
        let mut app_name = current_dir
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();
    
        index = 0;
        let mut language = "c++".to_string();
        let mut receving = true;
        let mut receving_language = true;
        for input in options {
            index += 1;
            match input.as_str() {
                "-n" | "--name" => {
                    let option_v = options_clone.get(index);
                    if option_v.is_none() {
                        println!("error: no provided '-n' option value.");
                        exit(1)
                    }
                    app_name = option_v.unwrap();
                    receving = true;
                }
                &_=> {
                    if receving == false { 
                        println!("error: unknown input.");
                        exit(1);
                    }
                    if receving_language {
                        language = input;
                        receving_language = false;
                    }
                }
            }
        }
    
        let mut file = std::fs::OpenOptions::
            new().
            write(true).
            create(true).
            open("manifest.json").
            unwrap();
        match write!(file, 
"{{
  \"name\": \"{}\",
  \"background_color\": \"#fff\",
  \"description\": \"\",
  \"language\": \"{}\"
  \"icons\": [],
  \"modules\": [],
  \"resources\": []
}}", app_name, language) {
            Ok(_) => (),
            Err(err) => {
                println!("error: {}", err.to_string());
                exit(1);
            }
        }
        println!("create app '{}' in current dir.", app_name);
        exit(0);
    }
    
    pub fn version() {
        println!("minimium 0.1.0");
    }
}
