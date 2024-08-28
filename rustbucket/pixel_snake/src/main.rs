use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect};
use ggez::timer;
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeGame {
    snake: VecDeque<(f32, f32)>,
    direction: Direction,
    food: (f32, f32),
    score: i32,
    screen_width: f32,
    screen_height: f32,
}

impl SnakeGame {
    fn new(screen_width: f32, screen_height: f32) -> Self {
        let initial_position = (screen_width / 2.0, screen_height / 2.0);
        let snake = VecDeque::from([initial_position]);
        let food = (100.0, 100.0);
        let direction = Direction::Right;

        SnakeGame {
            snake,
            direction,
            food,
            score: 0,
            screen_width,
            screen_height,
        }
    }

    fn reset_food(&mut self) {
        let new_x = rand::random::<f32>() * (self.screen_width - 6.0);
        let new_y = rand::random::<f32>() * (self.screen_height - 6.0);
        self.food = (new_x, new_y);
    }

    fn move_snake(&mut self) {
        let (head_x, head_y) = *self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y - 4.0),
            Direction::Down => (head_x, head_y + 4.0),
            Direction::Left => (head_x - 4.0, head_y),
            Direction::Right => (head_x + 4.0, head_y),
        };

        if new_head.0 < 0.0 || new_head.0 >= self.screen_width || new_head.1 < 0.0 || new_head.1 >= self.screen_height || self.snake.contains(&new_head) {
            self.snake.clear();
            self.snake.push_front((self.screen_width / 2.0, self.screen_height / 2.0));
            self.direction = Direction::Right;
            self.score = 0;
        } else {
            self.snake.push_front(new_head);

            if ((new_head.0 - self.food.0).abs() < 4.0) && ((new_head.1 - self.food.1).abs() < 4.0) {
                self.score += 1;
                self.reset_food();
            } else {
                self.snake.pop_back();
            }
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        if self.direction == Direction::Up && direction != Direction::Down
            || self.direction == Direction::Down && direction != Direction::Up
            || self.direction == Direction::Left && direction != Direction::Right
            || self.direction == Direction::Right && direction != Direction::Left
        {
            self.direction = direction;
        }
    }
}

impl EventHandler for SnakeGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 15;
        while timer::check_update_time(_ctx, DESIRED_FPS) {
            self.move_snake();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        // Draw the snake's head
        let head = self.snake.front().unwrap();
        let head_rect = Rect::new(head.0, head.1, 8.0, 8.0);
        let head_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), head_rect, Color::from_rgb(0, 255, 0))?;
        graphics::draw(ctx, &head_mesh, graphics::DrawParam::default())?;

        // Draw the snake's body
        for segment in self.snake.iter().skip(1) {
            let body_rect = Rect::new(segment.0, segment.1, 4.0, 4.0);
            let body_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), body_rect, Color::from_rgb(0, 200, 0))?;
            graphics::draw(ctx, &body_mesh, graphics::DrawParam::default())?;
        }

        // Draw the food
        let food_rect = Rect::new(self.food.0, self.food.1, 6.0, 6.0);
        let food_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), food_rect, Color::from_rgb(255, 0, 0))?;
        graphics::draw(ctx, &food_mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => self.change_direction(Direction::Up),
            KeyCode::Down => self.change_direction(Direction::Down),
            KeyCode::Left => self.change_direction(Direction::Left),
            KeyCode::Right => self.change_direction(Direction::Right),
            _ => (),
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("snake_game", "author")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let game = &mut SnakeGame::new(800.0, 600.0);
    event::run(ctx, event_loop, game)
}

