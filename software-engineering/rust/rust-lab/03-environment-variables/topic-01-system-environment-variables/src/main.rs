/*

ABOUT: System Environment Variables

System Environment Variables are variables that 
are defined and set outside of your program, by your Operating System.

Two common examples are `HOME` and `SHELL`

Open up a terminal and run this command:
echo $HOME

I get this output: 
/home/your-user-name

Open up a terminal and run this command:
echo $SHELL

I get this output because I am using ZSH as my shell.
/usr/bin/zsh

*/

// First bring the `env` module into scope.
use std::env;

fn main() {

    // The `var` function is used get information about 
    // an environment variable.
    // It will return a String if it succeeds and varError if it fails.

    // I'm using `.unwrap()` here to keep things simple.
    // If Rust can't find the environment variable called "HOME",
    // the program will crash immediately.
    let home_env = env::var("HOME").unwrap();
    
    let shell_env = env::var("SHELL").unwrap();

    println!("home_env: {home_env}");
    // home_env: /home/your-user-name
    
    println!("shell_env: {shell_env}");
    // shell_env: /usr/bin/your-shell
    
}
