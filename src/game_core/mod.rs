/*
This file is part of Tic-Tac-Toe game.
Copyright (C) 2020 Ahmad Amr Siam
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

mod computer_player;
mod controller;

pub use computer_player::*;
pub use controller::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    X,
    O,
    Empty,
}
impl Cell {
    /// returns the Cell contents as &str
    pub fn to_str(&self) -> &str {
        match self {
            Cell::X => "X",
            Cell::O => "O",
            Cell::Empty => "",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Level {
    Stupid,
    Normal,
    Smart,
}

pub mod rand {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn generate() -> u64 {
        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        seed = seed.wrapping_add(0xa0761d6478bd642f);
        let t: u128 = (seed).wrapping_mul(seed ^ 0xe7037ed1a0b428db);
        ((t >> 64) ^ t) as u64
    }
    pub fn generate_range(low: u64, high: u64) -> u64 {
        generate() % (high - low) + low
    }
}
