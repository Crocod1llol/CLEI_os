use std::thread;
use std::time::{Duration/*, SystemTime*/};

use dialoguer::Select;
use chrono::prelude::*;

//include the other file or smth
mod hacker_sim;

//the kind of "main" screen for the os
fn real_main() {
    //insert control char to clear the terminal screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    //the main collumns
    for _i in 0..25 {
        print!("###");

    }

    println!("\n\n                                CLEI_os");

    //display date and time
    let date_time: DateTime<Local> = Local::now();
    println!("\n                  {}\n", date_time);

    //prompt the user with all available actions
    let options = vec!["Main Menu", "Exit", "H@cker P()s3r $imulat0r"];

    let selection = Select::new().with_prompt("Main menu").items(&options).interact().expect("Error: Critical error");

    //check if required values match
    match selection {

     0 => {
         println!("Launching Main menu");

        //let the user "read" the text
        let duration = Duration::from_millis(500);
        thread::sleep(duration);
        real_main();
     },

    1 => {

        //Exit
        println!("\nSystem called poweroff\n");

        let duration = Duration::from_millis(550);
        thread::sleep(duration);

        for i in 0..20 {

            println!("Destroying service {}", i);
        }

        println!("Exit code: 0");

        let duration = Duration::from_millis(905);
        thread::sleep(duration);

        std::process::exit(0);
    },

    2 => {

        println!("Loading H@cker P()s3r $imulat0r");

        //fake load
        let duration = Duration::from_millis(1550);
        thread::sleep(duration);

        hacker_sim::real_main();
    },
     _ => {  println!("ERROR: VALUE OF selection BEING {} IS NOT DEFINED", selection);

        //let the user read the text
        let duration = Duration::from_millis(2000);
        thread::sleep(duration);

        real_main();
     }
    }

}

fn main() {
    //insert control char to clear the terminal screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    //startup idk
    println!("Loading stuff.");

    let duration = Duration::from_millis(500);
    thread::sleep(duration);

    println!("\nWelcome to CLEI os!\n");

    let duration = Duration::from_millis(500);
    thread::sleep(duration);

    println!("Starting services.");
    for i in 0..20 {

        println!("Running service with id {}", i);
    }

    //sleep for a bit to give the illusion that it actually loads shit
    let duration = Duration::from_millis(2000);
    thread::sleep(duration);

    real_main();
}
