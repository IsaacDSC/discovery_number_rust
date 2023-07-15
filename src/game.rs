use rand::Rng;
use std::io;

pub struct GamerDiscoveryNumber {}

impl GamerDiscoveryNumber {
    fn select_levels() -> i32 {
        println!("SELECT TO LEVEL \n");
        let mut select_level = String::new();
        let levels = ["Easy", "Medium", "Hard"];
        let radom_levels = [35, 50, 100];
        let mut counter: usize = 0;
        for level in levels {
            println!(
                "[ {counter} ] - {level} - Last Number: {}",
                &radom_levels[counter]
            );
            counter += 1;
        }
        io::stdin()
            .read_line(&mut select_level)
            .expect("Failed to read line");
        let select_level: usize = select_level.trim().parse().expect("Please type a number!");
        if select_level > levels.len() {
            return radom_levels[select_level];
        }
        println!("LEVEL SELECTED: {} \n", &levels[select_level]);
        return radom_levels[select_level];
    }

    fn generate_radom_number(max_radom_number: i32) -> i32 {
        return rand::thread_rng().gen_range(1..=max_radom_number);
    }

    fn is_win(secret_number: i32, input_number: i32) -> bool {
        return secret_number == input_number;
    }

    pub fn run() {
        let max_radom_number = GamerDiscoveryNumber::select_levels();
        for i in 0..3 {
            println!("Live: {}", 3 - i);
            let secret_number = GamerDiscoveryNumber::generate_radom_number(max_radom_number);
            let mut guess = String::new();
            println!("Guess the number!");
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            let guess: i32 = guess.trim().parse().expect("Please type a number!");
            if GamerDiscoveryNumber::is_win(secret_number, guess) {
                println!("Win Gamming assert secret number");
                return;
            }
            println!("Input Number: {guess} expect {secret_number}");
            println!("Try Again Number! \n");
        }
    }
}
