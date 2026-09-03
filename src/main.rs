use std::thread;
use std::time::{Duration/*, SystemTime*/};
use dialoguer::Select;

//

//the kind of "main" screen for the os
fn real_main(/*main_options: &str[]*/) {
    //insert control char to clear the terminal screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    //the main collumns
    for _i in 0..25 {
        print!("###");

    }

    println!("\n\n                                CLEI_os");

    //prompt the user with all available actions
    //let options: &str= ["1", "2", "3"];
    let options = vec!["1 - options", "2 - options", "3 - options"];

    let selection = Select::new().with_prompt("Actions available").items(&options).interact().expect("Error: Critical error");


}

fn main() {
    //insert control char to clear the terminal screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    //startup idk
    println!("Loading stuff.");

    println!("\nWelcome to CLEI os!\n");

    println!("Starting services.");
    for i in 0..20 {

        println!("Running service with id {}", i);
    }

    //sleep for a bit to give the illusion that it actually loads shit
    let duration = Duration::from_millis(2000);
    thread::sleep(duration);

    real_main();
}
