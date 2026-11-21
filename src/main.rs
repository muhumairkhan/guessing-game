use rand::RngExt;
use std::error::Error;
use std::io::{BufRead, stdin};

fn main() {
    println!("**Welcome to the Guessing Game**");

    // endless game loop
    loop {
        println!("Random number is generated.",);
        let mut rng: rand::prelude::ThreadRng = rand::rng();

        game_loop(rng);
    }
}

// Multiple Error types being returned so have to use
// Box trait that is common in all errors i.e std::error::Error
fn game_loop(mut rng: rand::prelude::ThreadRng) -> Result<(), Box<dyn Error>> {
    let win_num: i32 = rng.random_range(1..=100);

    // loop to give users multiple guesses
    loop {
        println!("Random: {}", win_num);

        println!("Enter a number to guess:");
        let mut line = String::new();

        // lock terminal to read user's input
        let mut stdin_lock = stdin().lock();

        // read user input into line
        stdin_lock.read_line(&mut line)?;

        // trim() trailing 'n from `line` and parse into i32
        let num: i32 = line.trim().parse()?;

        if num == win_num {
            let banner = r#"
__   __ ___   _   _    __        __ ___  _   _  _ 
\ \ / // _ \ | | | |   \ \      / /|_ _|| \ | || |
 \ V /| | | || | | |    \ \ /\ / /  | | |  \| || |

  | | | |_| || |_| |     \ V  V /   | | | |\  ||_|
  |_|  \___/  \___/       \_/\_/   |___||_| \_|(_)
"#;

            println!("{banner}");
            break;
        } else {
            println!("Wrong guess. Try again. \n");
        }
    }

    Ok(())
}
