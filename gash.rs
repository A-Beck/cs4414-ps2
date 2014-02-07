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
use std::io::File;
use std::io::signal::{Listener, Interrupt};


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
            let mut line: ~str = ~"";

            for &string in user_input.iter(){

                match string {

                    "<" => {
                        let file  = user_input[index+1].to_owned(); // get file name
                        let command = line.trim().to_owned();
                        let message = read_file(file);
                        println(message);

                        line = ~"";
                    }

                    ">" => {
			            let file = user_input[index + 1].to_owned();
			            let command = line.trim().to_owned();
			            let message = self.find_prog(command, true).to_owned();
                        write_file(file,message);
                        line = ~"";
                        user_input.iter().next();
                    }

                    "|" => {
                        let command = line.trim().to_owned();
                        let mut command2: ~str = ~"";
                        let length = user_input.len();

                        for i in range(index+1,length){
                            match user_input[i]{
                                "<" => {break;}
                                ">" =>{break;}
                                "|" => {break;}
                                _   => {command2 = command2 + " " + user_input[i];}
                            }
                        }
                        command2 = command2.trim().to_owned();
                        let message = self.find_prog(command, true).to_owned();
                        line = ~"";
                    }
                    _ => {
                        line = line + " " + string;
                    }
                }
                index += 1;
            }

            let result = self.find_prog(line.trim(),false);
            match result {
                ~"return" => {return}
                ~"continue" => {continue;}
                _          => {continue;}
            }
            
        }
    }
    
    fn find_prog(&mut self, cmd_line : &str, output: bool) -> ~str{
         let program = cmd_line.splitn(' ', 1).nth(0).expect("no program"); // get command

         match program {
                ""      =>  { return ~"continue"; }
                "exit"  =>  { return ~"return"; }
                "history" => {
                    self.log.push(cmd_line.to_owned());
                    return self.run_history(output);
                }
                "cd" => {
                    self.log.push(cmd_line.to_owned());
                    self.run_cd(cmd_line);
                }
                _       =>  { 
                    self.log.push(cmd_line.to_owned());
                    return self.run_cmdline(cmd_line, output); 
                }
            }
        ~"fine"
    }

    fn run_cmdline(&mut self, cmd_line: &str, output: bool) -> ~str {
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
	return ~"none";
	}
	else{
        if argv.len() > 0 {
            let program: ~str = argv.remove(0);
            return self.run_cmd(program, argv, output);
        }
	}
	return ~"none";
    }
    
    fn run_cmd(&mut self, program: &str, argv: &[~str], output: bool)->~str {
        if self.cmd_exists(program) {
	    if output == false {
            run::process_status(program, argv);
	    return ~"none";
	    }
	    else {
	    	let output = run::process_output(program, argv);
	  	let output_str = output.unwrap().output;
		return std::str::from_utf8(output_str).to_owned();
	    }
        } else {
            println!("{:s}: command not found", program);
        }
	return ~"none";
    }
    
    fn cmd_exists(&mut self, cmd_path: &str) -> bool {
        let ret = run::process_output("which", [cmd_path.to_owned()]);
        return ret.expect("exit code error.").status.success();
    }

    fn run_history(&mut self, output: bool)-> ~str{
	if output == false{
        	for i in range(0, self.log.len()) { 
            		println!("{}", self.log[i]);
        	}
		return ~"none";
	}
	else {
		let mut out: ~str = ~"";
		for i in range(0, self.log.len()){
			out = out + self.log[i].to_owned();
		}
		return out;
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
                    std::os::change_dir(&path);
               }
             None => {
                    println("$HOME is not set");
               }
         }
    }

}

fn write_file(filename: ~str, message: ~str){

    match File::create(&Path::new(filename)) {
        Some(mut file) => {
            file.write_str(message);
        }
        None =>{
            println("Opening file failed!");
        }
    }
}

fn read_file(filename: ~str) -> ~str{

    let path = Path::new(filename);
    match File::open(&path) {
        Some(file) => {
            let mut reader = BufferedReader::new(file);
            let input = reader.read_to_str();
            return input;
        }
        None =>{
            println("Opening file failed");
            return ~"";
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

    spawn(proc () {
    let mut listener = Listener::new();
        listener.register(Interrupt);
    loop{
        match listener.port.recv() {
            Interrupt => println!("Got interrupt"),
            _ => (),
        }
    }
   }); 
    
    match opt_cmd_line {
        Some(cmd_line) => {Shell::new("").run_cmdline(cmd_line, false);}
        None           => {Shell::new("gash > ").run();}
    }
}
