use rand::{Rng, thread_rng};
use entities::global_constants;
use crate::entities;
use crate::entities::global_constants::SCREEN_HEIGHT;

#[derive(Clone)]
pub struct Platform {
    x: f64,
    y: f64,
    width: i32,
    height: i32,
    has_coin: bool,
    coin_x: i32,
    coin_y: i32
}

pub fn platform_init() -> Platform {
    let platform: Platform = Platform {
        x: 0.0,
        y: 0.0,
        width: 0,
        height: 0,
        has_coin: false,
        coin_x: 0,
        coin_y: 0
    };
    platform
}

impl Platform {
    pub fn create(&mut self, index: i32) -> () {
        let mut rnd = rand::thread_rng();
        self.width = 100;
        self.height = 32;
        self.x = (rnd.gen_range(20..680)) as f64;
        self.y = (0 - self.height - (index * 100)) as f64;

        let coin_int: i32 = rnd.gen_range(0..4);
        if coin_int == 0 || index == 0 {
            self.has_coin = false;
        }
        else {
            self.has_coin = true;
        }

        self.coin_x = (self.x + (self.width) as f64 / 2.0 - 24.0 / 2.0) as i32;
        self.coin_y = (self.y - 24.0 - 5.0) as i32;
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_has_coin(&self) -> bool {
        self.has_coin
    }

    pub fn set_has_coin(&mut self, value: bool) -> () {
        self.has_coin = value;
    }

    pub fn get_coin_x(&self) -> i32 {
        self.coin_x
    }

    pub fn get_coin_y(&self) -> i32 {
        self.coin_y
    }

    pub fn update_position(&mut self) -> () {
        let mut rnd = rand::thread_rng();

        self.y += 1.0;

        self.coin_x = (self.x + self.width as f64 / 2.0 - 24.0 / 2.0) as i32;
        self.coin_y = (self.y - 24.0 - 5.0) as i32;

        if self.y > SCREEN_HEIGHT as f64 {
            self.x = (rnd.gen_range(20..680)) as f64;
            self.y = (0 - self.height) as f64;
            let coin_int: i32 = rnd.gen_range(0..4);

            if coin_int == 0 {
                self.has_coin = false;
            }
            else {
                self.has_coin = true;
            }
        }
    }
}