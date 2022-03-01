use raylib::prelude::*;
use crate::entities::global_constants::GRAVITY;
use crate::Platform;

pub struct Player {
    x: f64,
    y: f64,
    width: i32,
    height: i32,
    on_platform: bool,
    velocity: Vector2
}

pub fn player_init() -> Player {
    let player = Player {
        x: 0.0,
        y: 0.0,
        width: 0,
        height: 0,
        on_platform: false,
        velocity: Vector2::new(0.0,0.0)
    };
    player
}

impl Player{
    pub fn create(&mut self, x: f64, y: f64, width: i32, height: i32) -> () {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
        self.on_platform = false;
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) -> () {
        self.x = x as f64;
    }

    pub fn set_y(&mut self, y: i32) -> () {
        self.y = y as f64;
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn is_on_ground(&self) -> bool {
        self.on_platform
    }

    pub fn is_on_platform(&self) -> bool {
        self.is_on_ground()
    }

    pub fn set_on_platform(&mut self, result: bool) -> () {
        self.on_platform = result;
    }

    pub fn set_velocity(&mut self, x: f64, y: f64) -> () {
        self.velocity = Vector2::new(x as f32, y as f32);
    }

    pub fn get_velocity(&self) -> Vector2 {
        self.velocity
    }

    pub fn update_position(&mut self) -> () {
        self.x += self.velocity.x as f64;
        self.y += self.velocity.y as f64;

        if !self.is_on_ground() {
            self.velocity.y += GRAVITY as f32;
        }
        else {
            self.velocity = Vector2::new(0.0, 0.0);
        }

        if (self.x < 0.0) || ((self.x + self.width as f64) > 800.0) {
            self.velocity.x *= -1.0;
        }
    }
}