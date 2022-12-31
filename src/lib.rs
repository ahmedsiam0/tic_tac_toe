/*
This file is part of Tic-Tac-Toe game.
Copyright (C) 2020 Ahmad Amr Siam
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

mod game_core;
use crate::game_core::{Cell, ComputerPlayer, Controller, Level};

use std::io::{self, Write};
use std::process;

/// Starts the game and communicate with the player
pub fn run() {
    let mut controller = Controller::new();
    let mut computer_player = ComputerPlayer::new();
    show_intro();
    loop {
        input_mode(&mut controller, &mut computer_player);
        loop {
            if computer_player.get_active() {
                computer_player.play_turn(&mut controller);
                let winner = controller.get_winner();
                if winner != Cell::Empty {
                    show_playground(&controller);
                    println!("Winner is {}!\nGame Over!", winner.to_str());
                    break;
                }

                if !controller.can_continue() {
                    show_playground(&controller);
                    println!("No way to continue!\nGame Over!");
                    break;
                }
            }

            input_play(&mut controller);

            let winner = controller.get_winner();
            if winner != Cell::Empty {
                show_playground(&controller);
                println!("Winner is {}!\nGame Over!", winner.to_str());
                break;
            }

            if !controller.can_continue() {
                show_playground(&controller);
                println!("No way to continue!\nGame Over!");
                break;
            }
        }
    }
}
/// Inputs mode that player wants.
fn input_mode(c: &mut Controller, cp: &mut ComputerPlayer) {
    println!("\nType in Mode field: ");
    println!("    1 => play with computer.");
    println!("    2 => play with another person.");
    println!("    3 => last used option(or Default).");
    println!("    0 => exit the game.");
    let mut mode = String::new();
    c.initialize(); // initialize the game

    print!("Mode: ");
    io::stdout().flush().expect("flush failed!");

    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");

    let mode: i32 = match mode.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input only specified numbers!");
            input_mode(c, cp);
            return;
        }
    };

    match mode {
        0 => process::exit(0),
        1 => {
            cp.set_active(true);
            input_cp_info(c, cp);
        }
        2 => {
            cp.set_active(false);
        }
        3 => {}
        _ => {
            println!("Input only specified numbers!");
            input_mode(c, cp);
        }
    }
}
fn input_cp_info(c: &mut Controller, cp: &mut ComputerPlayer) {
    println!("\nType in Level field: ");
    println!("    0 => back to last step.");
    println!("    1 => play in stupid level.");
    println!("    2 => play in normal level.");
    println!("    3 => play in smart level.");
    loop {
        let mut level = String::new();
        print!("Level: ");
        io::stdout().flush().expect("flush failed!");

        io::stdin()
            .read_line(&mut level)
            .expect("Failed to read line");

        let level: i32 = match level.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input only specified numbers!");
                continue;
            }
        };

        match level {
            0 => {
                input_mode(c, cp);
                return;
            }
            1 => cp.set_level(Level::Stupid),
            2 => cp.set_level(Level::Normal),
            3 => cp.set_level(Level::Smart),
            _ => {
                println!("Input only specified numbers!");
                continue;
            }
        }
        break;
    }
    println!("\nType in Player field: ");
    println!("    0 => back to last step.");
    println!("    1 => you will be Player X.");
    println!("    2 => you will be Player O.");
    loop {
        let mut player_opponent = String::new();
        print!("Player: ");
        io::stdout().flush().expect("flush failed!");

        io::stdin()
            .read_line(&mut player_opponent)
            .expect("Failed to read line");

        let player_opponent: i32 = match player_opponent.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input only specified numbers!");
                continue;
            }
        };

        match player_opponent {
            0 => {
                input_cp_info(c, cp);
                return;
            }
            1 => cp.set_player_turn(Cell::O),
            2 => cp.set_player_turn(Cell::X),
            _ => {
                println!("Input only specified numbers!");
                continue;
            }
        }
        break;
    }
}
/// Inputs player choice and plays it.
fn input_play(c: &mut Controller) {
    let mut choice = String::new();

    show_playground(c);
    print!("Player {}: ", c.get_player_turn().to_str());
    io::stdout().flush().expect("flush failed!");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input only specified numbers!");
            return;
        }
    };

    let (row, column);
    match choice {
        0 => process::exit(0),
        1 => {
            row = 1;
            column = 1;
        }
        2 => {
            row = 2;
            column = 1;
        }
        3 => {
            row = 3;
            column = 1;
        }
        4 => {
            row = 1;
            column = 2;
        }
        5 => {
            row = 2;
            column = 2;
        }
        6 => {
            row = 3;
            column = 2;
        }
        7 => {
            row = 1;
            column = 3;
        }
        8 => {
            row = 2;
            column = 3;
        }
        9 => {
            row = 3;
            column = 3;
        }
        _ => {
            println!("Input only specified numbers!");
            return;
        }
    }

    if !c.is_free(row, column) {
        println!("Choose empty location!");
    } else {
        c.play_turn(row, column);
    }
}
/// Prints the playground.
pub fn show_playground(c: &Controller) {
    println!("-------------");
    for column in 1..4 {
        for row in 1..4 {
            if row == 1 {
                print!("| ");
            }
            if c.get_content(row, column) != Cell::Empty {
                print!("{} | ", c.get_content(row, column).to_str());
            } else {
                print!("{} | ", (column - 1) * 3 + row);
            }
        }
        println!();
        println!("-------------");
    }
}
/// Prints how to play the game.
pub fn show_intro() {
    println!("Copyright (C) 2020 Ahmad Amr Siam.\n");
    println!("Hello in Tic Tac Toe game!");
    println!("When the game starts type the number of the cell");
    println!("you want to fill or 0 to exit the game.");
}
