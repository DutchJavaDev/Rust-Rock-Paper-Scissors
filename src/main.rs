extern crate rand;

use std::io;
use rand::Rng;

#[derive(Debug)]
#[derive(PartialEq)]
enum Winner {
    None,
    Robot,
    Draw,
    You
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Choices {
    None,
    Rock,
    Paper,
    Scissors
}

macro_rules! writeln {
    () => {
        println!();
    };

    ($txt:expr) => {
        println!("{}",$txt);
    };
}

fn _get_user_input() -> String {

    let mut _input = String::new();

    match io::stdin().read_line(&mut _input){
        Ok(_n) => println!(""),
        Err(error) => println!("error: {}", error),
    }

    _input.to_lowercase().trim().to_owned()
}

fn _get_user_choice(_string:String) -> Choices {

    let mut _number : i32 = match _string.parse() {
        Ok(num) => num,
        Err(_) => { -1 }
    };

    match _number {
        -1 | std::i32::MIN..=-2i32 | 0i32 | 4i32..=std::i32::MAX => Choices::None,

        1 => Choices::Rock,

        2 => Choices::Paper,

        3 => Choices::Scissors
    }
}

fn _get_robot_choice() -> Choices {

    let mut _number = rand::thread_rng().gen_range(0,2);

    match _number { 
        0 => Choices::Rock,

        1 => Choices::Paper,

        2 => Choices::Scissors,

        std::i32::MIN..=-2i32 | 4i32..=std::i32::MAX | -1i32 | 3i32 => Choices::None,
    }
}

fn _show_user_choice() {

    writeln!("Type: rock, paper or scissors");

    writeln!("Or type 1 for rock, 2 for paper and 3 for scissors");

    writeln!("Type exit or e to exit");

    writeln!();
}

fn _run_game(_robot_choice:Choices, _user_choice:Choices) -> Winner {

    let mut _winner : Winner = Winner::None;

    match _robot_choice {

        Choices::None => { 
            _winner = Winner::None
        }

        Choices::Rock => {
            _winner = if _user_choice == Choices::Rock { Winner::Draw }

                else if _user_choice == Choices::Paper { Winner::You }

                else if _user_choice == Choices::Scissors { Winner::Robot }

                else { Winner::None } 
        }

        Choices::Paper => {
            _winner = if _user_choice == Choices::Rock { Winner::Robot }

                else if _user_choice == Choices::Paper { Winner::Draw }

                else if _user_choice == Choices::Scissors { Winner::You }

                else { Winner::None } 
        }

        Choices::Scissors => {
            _winner = if _user_choice == Choices::Rock { Winner::You }

                else if _user_choice == Choices::Paper { Winner::Robot }

                else if _user_choice == Choices::Scissors { Winner::Draw }

                else { Winner::None } 
        }
    }

    _winner
}

fn main(){

    writeln!("Welcome to Rock Paper and Scissors game");

    writeln!();

    loop {
        
        _show_user_choice();

        let _user_input = _get_user_input();

        if _user_input == "e" || _user_input == "exit" {
            break;
        }

        let _robot = _get_robot_choice();

        let _user = _get_user_choice(_user_input);

        match _user {

            Choices::None => { writeln!("Please enter a valid number!") }

            Choices::Rock | Choices::Paper | Choices::Scissors => {

                match _run_game(_robot, _user) {

                    Winner::Draw => {
                        writeln!("Its a draw!")
                    }

                    Winner::Robot => {
                        writeln!("You lose!")
                    }

                    Winner::You => {
                        writeln!("You won!")
                    }

                    Winner::None => {
                        writeln!("Hehhe not gonna happen")
                    }
                };

                writeln!()
            }

        };
    }
    writeln!("End of game");
}