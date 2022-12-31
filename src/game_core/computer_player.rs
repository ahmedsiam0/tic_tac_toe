/*
This file is part of Tic-Tac-Toe game.
Copyright (C) 2020 Ahmad Amr Siam
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use crate::game_core::{rand, Cell, Controller, Level};

pub struct ComputerPlayer {
    player_turn: Cell,
    active: bool,
    level: Level,
}

impl ComputerPlayer {
    pub fn new() -> ComputerPlayer {
        ComputerPlayer {
            player_turn: Cell::O,
            active: false,
            level: Level::Stupid,
        }
    }

    pub fn set_player_turn(&mut self, player_turn: Cell) {
        self.player_turn = player_turn;
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }
    pub fn get_active(&self) -> bool {
        self.active
    }
    pub fn get_opponent(&self) -> Cell {
        match self.player_turn {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            _ => Cell::Empty,
        }
    }

    pub fn play_turn(&self, controller: &mut Controller) {
        if !self.active || self.player_turn != controller.get_player_turn() {
            return;
        }

        match self.level {
            Level::Smart => {
                let (row, column) = ComputerPlayer::normal_play(controller, self.player_turn);
                if row != 0 {
                    controller.play_turn(row, column);
                    return;
                }
                let (row, column) = ComputerPlayer::normal_play(controller, self.get_opponent());
                if row != 0 {
                    controller.play_turn(row, column);
                    return;
                }
                let (row, column) = self.smart_play(controller);
                if row != 0 {
                    controller.play_turn(row, column);
                    return;
                }
                let (row, column) = ComputerPlayer::stupid_play(controller);
                controller.play_turn(row, column);
            }
            Level::Normal => {
                let (row, column) = ComputerPlayer::normal_play(controller, self.player_turn);
                if row != 0 {
                    controller.play_turn(row, column);
                    return;
                }
                let (row, column) = ComputerPlayer::normal_play(controller, self.get_opponent());
                if row != 0 {
                    controller.play_turn(row, column);
                    return;
                }
                let (row, column) = ComputerPlayer::stupid_play(controller);
                controller.play_turn(row, column);
            }
            Level::Stupid => {
                let (row, column) = ComputerPlayer::stupid_play(controller);
                controller.play_turn(row, column);
            }
        }
    }

    fn stupid_play(controller: &mut Controller) -> (usize, usize) {
        loop {
            let row = rand::generate_range(1, 4) as usize;
            let column = rand::generate_range(1, 4) as usize;

            if controller.is_free(row, column) {
                return (row, column);
            }
        }
    }
    fn normal_play(controller: &mut Controller, player: Cell) -> (usize, usize) {
        for i in 1..4 {
            for j in 1..4 {
                if controller.get_content(i, j) != player {
                    continue;
                }
                if controller.get_content(i, j) == controller.get_content(i + 1, j) {
                    if i + 2 < 4 && controller.get_content(i + 2, j) == Cell::Empty {
                        return (i + 2, j);
                    }
                    if i - 1 > 0 && controller.get_content(i - 1, j) == Cell::Empty {
                        return (i - 1, j);
                    }
                }
                if controller.get_content(i, j) == controller.get_content(i, j + 1) {
                    if j + 2 < 4 && controller.get_content(i, j + 2) == Cell::Empty {
                        return (i, j + 2);
                    }
                    if j - 1 > 0 && controller.get_content(i, j - 1) == Cell::Empty {
                        return (i, j - 1);
                    }
                }
                if controller.get_content(i, j) == controller.get_content(i + 2, j)
                    && controller.get_content(i + 1, j) == Cell::Empty
                {
                    return (i + 1, j);
                }
                if controller.get_content(i, j) == controller.get_content(i, j + 2)
                    && controller.get_content(i, j + 1) == Cell::Empty
                {
                    return (i, j + 1);
                }
            }
        }
        let top_left = controller.get_content(1, 1);
        let top_right = controller.get_content(3, 1);
        let bottom_left = controller.get_content(1, 3);
        let bottom_right = controller.get_content(3, 3);
        let center = controller.get_content(2, 2);

        if top_left == player && top_left == center && bottom_right == Cell::Empty {
            return (3, 3); // bottom_right
        }
        if center == player && center == bottom_right && top_left == Cell::Empty {
            return (1, 1); // top_left
        }
        if top_left == player && top_left == bottom_right && center == Cell::Empty {
            return (2, 2); // center
        }
        if bottom_left == player && bottom_left == center && top_right == Cell::Empty {
            return (3, 1); // top_right
        }
        if top_right == player && top_right == center && bottom_left == Cell::Empty {
            return (1, 3); // bottom_left
        }
        if bottom_left == player && bottom_left == top_right && center == Cell::Empty {
            return (2, 2); // center
        }
        (0, 0)
    }
    fn smart_play(&self, controller: &mut Controller) -> (usize, usize) {
        let top_left = controller.get_content(1, 1);
        let top_center = controller.get_content(2, 1);
        let top_right = controller.get_content(3, 1);

        let bottom_left = controller.get_content(1, 3);
        let bottom_center = controller.get_content(2, 3);
        let bottom_right = controller.get_content(3, 3);

        let center_left = controller.get_content(1, 2);
        let center = controller.get_content(2, 2);
        let center_right = controller.get_content(3, 2);

        // Main Move
        if center == Cell::Empty {
            return (2, 2); // center
        }

        // Defense

        // From first strategy
        if bottom_left == Cell::Empty
            && top_right == Cell::Empty
            && bottom_right == Cell::Empty
            && top_left == Cell::Empty
        {
            return (1, 1); // top_left
        }

        // From second strategy
        if top_left == self.player_turn
            && center == self.get_opponent()
            && bottom_right == self.get_opponent()
            && top_right == Cell::Empty
        {
            return (3, 1); // top_right
        }
        // From special case
        if top_left == self.get_opponent()
            && center == self.player_turn
            && bottom_right == self.get_opponent()
            && center_right == Cell::Empty
        {
            return (3, 2); // center_right
        }
        if top_right == self.get_opponent()
            && center == self.player_turn
            && bottom_left == self.get_opponent()
            && center_left == Cell::Empty
        {
            return (1, 2); // center_left
        }

        // Attack

        if center != self.player_turn {
            return (0, 0);
        }

        // First strategy
        if top_left == self.player_turn
            && center_left == self.get_opponent()
            && top_right == Cell::Empty
        {
            return (3, 1); // top_right
        }
        if top_left == self.player_turn
            && (top_center == self.get_opponent()
                || bottom_center == self.get_opponent()
                || center_right == self.get_opponent())
            && bottom_left == Cell::Empty
        {
            return (1, 3); // bottom_left
        }

        // Second strategy
        if bottom_left == self.get_opponent()
            && top_right == Cell::Empty
            && bottom_right == Cell::Empty
            && top_left == Cell::Empty
        {
            return (3, 1); // top_right
        }
        if bottom_left == Cell::Empty
            && top_right == self.get_opponent()
            && bottom_right == Cell::Empty
            && top_left == Cell::Empty
        {
            return (1, 3); // bottom_left
        }
        if bottom_left == Cell::Empty
            && top_right == Cell::Empty
            && bottom_right == self.get_opponent()
            && top_left == Cell::Empty
        {
            return (1, 1); // top_left
        }
        if bottom_left == Cell::Empty
            && top_right == Cell::Empty
            && bottom_right == Cell::Empty
            && top_left == self.get_opponent()
        {
            return (3, 3); // bottom_right
        }
        if top_left == self.player_turn && bottom_right == self.get_opponent() {
            if center_left == self.get_opponent() && top_right == Cell::Empty {
                return (3, 1); // top_right
            }
            if top_center == self.get_opponent() && bottom_left == Cell::Empty {
                return (1, 3); // bottom_left
            }
        }
        if bottom_right == self.player_turn && top_left == self.get_opponent() {
            if bottom_center == self.get_opponent() && top_right == Cell::Empty {
                return (3, 1); // top_right
            }
            if center_right == self.get_opponent() && bottom_left == Cell::Empty {
                return (1, 3); // bottom_left
            }
        }
        if top_right == self.player_turn && bottom_left == self.get_opponent() {
            if center_left == self.get_opponent() && bottom_right == Cell::Empty {
                return (3, 3); // bottom_right
            }
            if bottom_center == self.get_opponent() && top_left == Cell::Empty {
                return (1, 1); // top_left
            }
            if top_center == self.get_opponent() && bottom_right == Cell::Empty {
                return (3, 3); // bottom_right
            }
            if center_right == self.get_opponent() && top_left == Cell::Empty {
                return (1, 1); // top_left
            }
        }
        (0, 0)
    }
}
