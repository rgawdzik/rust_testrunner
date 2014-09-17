extern crate colorize;
use colorize::AnsiColor;

use std::{io,os};
use std::io::File;
use std::io::fs;
use std::io::process::Command;

fn main() {
    let args = os::args();
    let mut it = args.iter();
    it.next(); //We don't care about the name of our current process.
    loop {
        match it.next() {
            Some(str) => {
                match str.as_slice() {
                    "--help" | "-h" => {
                        return help();
                    },
                    "init" | "--i" => {
                        return init();
                     },
                     _ => {
                        return throw_error(format!("Found invalid argument: {}.", str))
                    }
                }
            },
            None => { return run_tests(); }
        }
    }
}

fn throw_error(msg: String) {
    print!("{}", "ERROR: ".to_string().red());
    println!("{}", msg);
}

fn init() {
    match fs::mkdir(&Path::new("fixtures/"), io::UserRWX) {
        Err(_) => return throw_error(format!("fixtures/ could not be created, it already exists!")),
        Ok(_)    => {}
    }

    let test_sh = include_str!("test.sh");
    let mut file = File::create(&Path::new("test.sh"));
    file.write_str(test_sh);


    println!("{}","Initialization Finished!".to_string().b_greyb().black());
    println!("fixtures/ and test.sh have been created.");
}

fn run_tests() {
    let mut tests: Vec<&str>;
    let mut file_ref: String = String::new();
    match File::open(&Path::new("test.sh")).read_to_string() {
        Ok(file) => {
            file_ref = file;
            tests = file_ref.as_slice().splitn(-1, '\n').collect();
        },
        Err(_) => return throw_error(format!("test.sh not found!"))
    }

    //Filter out blank lines and pure comments
    let clean_tests: Vec<&str> = tests.iter()
            .filter(|str| str.len() > 0 && str.char_at(0) != '#' )
            .map(|str| *str).collect();

    let mut i = 1i;
    let mut passed = 0u;
    let mut errors = String::new();

    println!("\n{}","TEST RESULTS".to_string().b_greyb().black());

    for &test in clean_tests.iter() {
        let mut process = match Command::new("bash").spawn() {
            Err(_) => return throw_error(format!("Can't open bash!")),
            Ok(process) => process
        };

        {
            let mut stdin = process.stdin.take().unwrap();
            match stdin.write_str(test) {
                Err(_) => return throw_error(format!("Can't give stdin to bash!")),
                Ok(_) => {}
            }
        } //Closes stdin.

        match process.stdout.as_mut().unwrap().read_to_string() {
            Err(_) => {
                errors.push_str(format!("\n{}", "FAIL: ".to_string().red()).as_slice());
                errors.push_str(format!("{}/{}", i, clean_tests.len()).as_slice());
                errors.push_str(format!("\nI/O ERROR!").as_slice());
            },
            Ok(msg) => {
                if msg.len() == 0 {
                    print!("{}", ".".to_string().green());
                    passed += 1;
                }
                else {
                    print!("{}", "E".to_string().red());
                    errors.push_str(format!("\n{}", "FAIL: ".to_string().red()).as_slice());
                    errors.push_str(format!("{}/{}", i, clean_tests.len()).as_slice());
                    errors.push_str(format!("\n{}", msg).as_slice());
                }
            }
        }
        i += 1;
    }
    if passed < clean_tests.len() {
        print!("\n{}","FAILING TESTS".to_string().b_greyb().black());
        println!("{}", errors);
    }
    println!("\nTests Passed: {}/{}", passed, clean_tests.len());

}

fn help() {
    print!("{}","--help, -h  ".to_string().b_greyb().black()); println!(" Opens this help." );
    print!("{}","init, --init".to_string().b_greyb().black()); println!(" Initializes a \
        testing suite in the current directory.");
    print!("{}","*no args*   ".to_string().b_greyb().black()); println!(" Runs the test \
        suite found in tests.sh");
}
