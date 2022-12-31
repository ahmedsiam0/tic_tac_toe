/*
This file is part of Tic-Tac-Toe game.
Copyright (C) 2020 Ahmad Amr Siam
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use crate::game_core::Cell;

/// Controls the game and contains the playground and player turn
pub struct Controller {
    playground: [[Cell; 3]; 3],
    player_turn: Cell,
}
impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
impl Controller {
    /// Creates new Controller and returns it
    pub fn new() -> Controller {
        Controller {
            playground: [[Cell::Empty; 3]; 3],
            player_turn: Cell::X,
        }
    }
    /// Initialize the Controller
    pub fn initialize(&mut self) {
        *self = Controller::new();
    }
    /// Takes row and column parameters and returns the content of the cell with this location
    pub fn get_content(&self, row: usize, column: usize) -> Cell {
        if row > 3 || column > 3 || row < 1 || column < 1 {
            return Cell::Empty;
        }
        self.playground[row - 1][column - 1]
    }
    /// returns which player will play next
    pub fn get_player_turn(&self) -> Cell {
        self.player_turn
    }
    /// Puts X or O in the cell that is in row and column of the parameters
    pub fn play_turn(&mut self, row: usize, column: usize) {
        match self.player_turn {
            Cell::X => self.playground[row - 1][column - 1] = Cell::X,
            Cell::O => self.playground[row - 1][column - 1] = Cell::O,
            _ => {}
        }
        self.change_turn();
    }
    /// Changes player turn
    fn change_turn(&mut self) {
        match self.player_turn {
            Cell::X => self.player_turn = Cell::O,
            Cell::O => self.player_turn = Cell::X,
            _ => {}
        }
    }
    /// Checks if a certain cell is free(empty) or no and returns the answer
    pub fn is_free(&self, row: usize, column: usize) -> bool {
        if row < 1
            || column < 1
            || row > 3
            || column > 3
            || self.playground[row - 1][column - 1] != Cell::Empty
        {
            return false;
        }
        true
    }
    /// Checks if there is a chance to continue playing or no and returns the answer
    pub fn can_continue(&self) -> bool {
        for row in self.playground.iter() {
            for cell in row.iter() {
                if cell == &Cell::Empty {
                    return true;
                }
            }
        }
        false
    }
    /// Checks if there is a winner or no if yes it returns the winner else it returns Cell::Empty
    pub fn get_winner(&self) -> Cell {
        for i in 0..3 {
            if self.playground[0][i] != Cell::Empty
                && self.playground[0][i] == self.playground[1][i]
                && self.playground[0][i] == self.playground[2][i]
            {
                return self.playground[0][i];
            }
        }
        for i in 0..3 {
            if self.playground[i][0] != Cell::Empty
                && self.playground[i][0] == self.playground[i][1]
                && self.playground[i][0] == self.playground[i][2]
            {
                return self.playground[i][0];
            }
        }
        if self.playground[0][0] != Cell::Empty
            && self.playground[0][0] == self.playground[1][1]
            && self.playground[0][0] == self.playground[2][2]
        {
            return self.playground[0][0];
        }
        if self.playground[0][2] != Cell::Empty
            && self.playground[0][2] == self.playground[1][1]
            && self.playground[0][2] == self.playground[2][0]
        {
            return self.playground[0][2];
        }
        Cell::Empty
    }
}
