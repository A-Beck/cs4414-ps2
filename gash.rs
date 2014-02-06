//
// gash.rs
//
// Starting code for PS2
// Running on Rust 0.9
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//

extern mod extra;

use std::{io, run, os};
use std::io::buffered::BufferedReader;
use std::io::stdin;
use extra::getopts;

struct Shell {
    cmd_prompt: ~str,
    log:        ~[~str],
}

impl Shell {

    fn new(prompt_str: &str) -> Shell {
        Shell {
            cmd_prompt: prompt_str.to_owned(),
            log: ~[],
        }
    }
    
    fn run(&mut self) {
        let mut stdin = BufferedReader::new(stdin());
        
        loop {
            print(self.cmd_prompt); // prints 'gash >'
            io::stdio::flush();
            
            let line = stdin.read_line().unwrap(); // reads whats on the line
            let cmd_line = line.trim().to_owned();  // removes leading and trailing whitespace
            let user_input: ~[&str] = cmd_line.split(' ').collect();
            let mut index: uint = 0;

            for &string in user_input.iter(){
                let mut prog : ~str;    // saves command
                let mut input : ~str;
                let mut index1 : uint;
                let mut index2 : uint;
                let mut index3 : uint;
                let mut index4 : uint;

                match string {
                    "<" => {
                        let file  = user_input[index+1].to_owned(); // get file name
                        println(file);
                    }
                    ">" => {

                    }
                    "|" => {

                    }
                    _ => {

                    }
                }
                index += 1;
            }


            let result = self.find_prog(cmd_line);
            match result {
                ~"return" => {return}
                ~"continue" => {continue;}
                _          => {continue;}
            }
            
        }
    }
    
    fn find_prog(&mut self, cmd_line : &str) -> ~str{
         let program = cmd_line.splitn(' ', 1).nth(0).expect("no program"); // get command

         match program {
                ""      =>  { return ~"continue"; }
                "exit"  =>  { return ~"return"; }
                "history" => {
                    self.log.push(cmd_line.to_owned());
                    self.run_history();
                }
                "cd" => {
                    self.log.push(cmd_line.to_owned());
                    self.run_cd(cmd_line);
                }
                _       =>  { 
                    self.log.push(cmd_line.to_owned());
                    self.run_cmdline(cmd_line); 
                }
            }
        ~"fine"
    }

    fn run_cmdline(&mut self, cmd_line: &str) {
        let mut argv: ~[~str] =
            cmd_line.split(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();

	let length = argv.len()-1;

    	if argv[length] == ~"&" {
	    argv.remove(length);
            let program: ~str = argv.remove(0);
	    let argv2 = argv.clone();
	    if self.cmd_exists(program){
	
		spawn(proc() {run::process_status(program, argv2);});
		
	    }
            else {
		println!("{:s}: command not found", program);
            }
	}
	else{
        if argv.len() > 0 {
            let program: ~str = argv.remove(0);
            self.run_cmd(program, argv);
        }
	}
    }
    
    fn run_cmd(&mut self, program: &str, argv: &[~str]) {
        if self.cmd_exists(program) {
            run::process_status(program, argv);
        } else {
            println!("{:s}: command not found", program);
        }
    }
    
    fn cmd_exists(&mut self, cmd_path: &str) -> bool {
        let ret = run::process_output("which", [cmd_path.to_owned()]);
        return ret.expect("exit code error.").status.success();
    }

    fn run_history(&mut self){
        for i in range(0, self.log.len()) { 
            println!("{}", self.log[i]);
        }
    }

    fn run_cd(&mut self, cmd_line: &str) {
         let mut argv: ~[~str] =
            cmd_line.split(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
    
        if argv.len() > 0 {

            if argv.len() == 1 {
                self.go_to_home_dir();
                return ;
            }

            let string: ~str = argv.remove(1);
            if string == ~"$HOME" || string == ~"$home"{
                self.go_to_home_dir();
                return ;
            }
            let path = ~Path::new(string);
            if (path.exists()){
                let success = std::os::change_dir(path);
                if !success {
                    println("Invalid path");
                }
            }
        }
        else {
            println("Invalid input");
        }
    }

    fn go_to_home_dir(&mut self){
        match std::os::homedir() {
            Some(path) => {
                    std::os::change_dir(~path);
               }
             None => {
                    println("$HOME is not set");
               }
         }
    }

}

fn get_cmdline_from_args() -> Option<~str> {
    /* Begin processing program arguments and initiate the parameters. */
    let args = os::args();
    
    let opts = ~[
        getopts::optopt("c")
    ];
    
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };
    
    if matches.opt_present("c") {
        let cmd_str = match matches.opt_str("c") {
                                                Some(cmd_str) => {cmd_str.to_owned()}, 
                                                None => {~""}
                                              };
        return Some(cmd_str);
    } else {
        return None;
    }
}

fn main() {
    let opt_cmd_line = get_cmdline_from_args();
    
    match opt_cmd_line {
        Some(cmd_line) => Shell::new("").run_cmdline(cmd_line),
        None           => Shell::new("gash > ").run()
    }
}
