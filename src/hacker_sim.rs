use std::time::Duration;
use std::thread;
use std::io;

//include main so we can call the main menu

pub fn real_main() {

    //insert control char to clear the terminal screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("W3lcome");


    println!("Select an option:");
    println!("1 - spamming text");
    println!("2 - free type");
    println!("3 - exit");

    let mut selec: String = Default::default();

    io::stdin().read_line(&mut selec).expect("ERROR: UNABLE TO READ. SELF DESTRUCT");

    let trim_selec = selec.trim();

    if trim_selec == "1" {

        let duration = Duration::from_millis(450);
        thread::sleep(duration);
        
        spam_text();
    } else if trim_selec == "2" {

        let duration = Duration::from_millis(450);
        thread::sleep(duration);

        free_type();
    } else if trim_selec == "3" {
        //return back
        //the main.rs can be used as crate::*function*
        crate::real_main();
        return;
    } else {

        println!("INVALID OPTION");

        let duration = Duration::from_millis(300);
        thread::sleep(duration);

        real_main();
    }

}

fn spam_text() {

    println!("Every 1 million line, it will prompt you for user input, type stop to exit or dont type anything to keep going");

    let mut selec: String = Default::default();
    io::stdin().read_line(&mut selec).expect("ERROR: UNABLE TO READ. SELF DESTRUCT");

    let trim_selec = selec.trim();

    if trim_selec == "stop" {
        real_main()
    }

    for _i in 0..1000000 {

        print!("2315486258^%*@&%^#@%$&^@3kgjhg2j3h4I!^(*#@&^$(*&!@3iugkhgkjh");
    }

    spam_text();
}

fn free_type() {

    println!("In this mod3, you can type to infinity, just type stop() when you want to exit.");

    loop {

        let mut selec: String = Default::default();
        io::stdin().read_line(&mut selec).expect("ERROR: UNABLE TO READ");

        let trim_selec = selec.trim();

        if trim_selec == "stop()" {

            real_main();
            return; 
        }
    }
}
