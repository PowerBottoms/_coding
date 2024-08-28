extern crate pancurses;
extern crate rand;

use pancurses::{initscr, endwin, noecho, Window};
use rand::Rng;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeGame {
    window: Window,
    snake: VecDeque<(i32, i32)>,
    direction: Direction,
    food: (i32, i32),
    score: i32,
}

impl SnakeGame {
    fn new() -> Self {
        let window = initscr();
        window.timeout(100); // Set input delay
        window.keypad(true);
        noecho();

        let initial_position = (10, 10);
        let snake = VecDeque::from([initial_position]);
        let food = (5, 5);
        let direction = Direction::Right;

        SnakeGame {
            window,
            snake,
            direction,
            food,
            score: 0,
        }
    }

    fn run(&mut self) {
        loop {
            self.render();
            self.handle_input();
            if self.update() {
                break;
            }
        }
        endwin();
        println!("Game Over! Your score: {}", self.score);
    }

    fn render(&self) {
        self.window.clear();

        for &(y, x) in &self.snake {
            self.window.mvaddch(y, x, '#');
        }

        self.window.mvaddch(self.food.0, self.food.1, '*');

        self.window.mv(0, 0);
        self.window.printw(format!("Score: {}", self.score));
        self.window.refresh();
    }

    fn handle_input(&mut self) {
        match self.window.getch() {
            Some(pancurses::Input::KeyUp) if self.direction != Direction::Down => {
                self.direction = Direction::Up;
            }
            Some(pancurses::Input::KeyDown) if self.direction != Direction::Up => {
                self.direction = Direction::Down;
            }
            Some(pancurses::Input::KeyLeft) if self.direction != Direction::Right => {
                self.direction = Direction::Left;
            }
            Some(pancurses::Input::KeyRight) if self.direction != Direction::Left => {
                self.direction = Direction::Right;
            }
            _ => {}
        }
    }

    fn update(&mut self) -> bool {
        let (y, x) = *self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
            Direction::Right => (y, x + 1),
        };

        if new_head.0 < 0 || new_head.1 < 0 || self.snake.contains(&new_head) {
            return true; // Game over
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.score += 1;
            self.food = (
                rand::thread_rng().gen_range(0..20),
                rand::thread_rng().gen_range(0..20),
            );
        } else {
            self.snake.pop_back();
        }

        false
    }
}

fn main() {
    let mut game = SnakeGame::new();
    game.run();
}

