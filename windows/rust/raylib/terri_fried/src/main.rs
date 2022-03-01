mod entities;

use raylib::audio::Sound;
use raylib::color::{Color, rcolor};
use raylib::drawing::RaylibDraw;
use entities:: {
    platform::Platform,
    player::Player,
    global_constants:: {
        SCREEN_HEIGHT,
        SCREEN_WIDTH
    }
};

use raylib::ffi::{
    LoadStorageValue,
    SaveStorageValue
};
use raylib::misc::AsF32;
use raylib::prelude::MouseButton::*;
use raylib::prelude::*;
use raylib::prelude::{FontLoadEx, Image, Texture2D, Vector2, Vector4};
use raylib::text::Font;
use crate::entities::platform::platform_init;
use crate::entities::player::player_init;

fn add_score(amount: i32, score_int: &mut i32, highscore_int: &mut i32, score: &mut String, highscore: &mut String) -> () {
    *score_int += amount;

    if *score_int < 10 {
        *score = format!("00{}", *score_int);
    }
    else if *score_int < 100 {
        *score = format!("0{}", *score_int);
    }
    else {
        *score = format!("{}", *score_int);
    }

    if *score_int > *highscore_int {
        *highscore_int = *score_int;
        *highscore = format!("BEST: {}", highscore_int);
    }
}

fn reset_score(score_int: &mut i32, highscore_int: &mut i32, score: &mut String) -> () {
    *score_int = 0;
    *score = format!("00{}", *score_int);
    unsafe {
        SaveStorageValue(0, *highscore_int);
    }
}

fn reset_game(platforms: &mut [Platform; 4], player: &mut Player, score_int: &mut i32, highscore_int: &mut i32, score: &mut String) -> () {
    reset_score(score_int, highscore_int, score);

    let mut i: i32 = 0;
    while (i as usize) < platforms.len() {
        platforms[i as usize].create(i);
        i += 1;
    }
    i=0;
    player.set_velocity(0.0, 0.0);
    player.set_x((platforms[0].get_x() + (platforms[0].get_width() / 2 - 26 / 2) as f64) as i32);
    player.set_y((platforms[0].get_y() - (player.get_height() as f64)) as i32);
}

fn check_player_collision(platforms: &mut [Platform; 4], player: &mut Player, score_int: &mut i32, highscore_int: &mut i32, score: &mut String, highscore: &mut String, play_coin_fx: &mut bool) {
    let mut on_platform: bool = false;

    let mut i: i32 = 0;
    while i < 4 {
        if platforms[i as usize].get_has_coin() && player.get_x() + player.get_width() as f64 - 3.0 > platforms[i as usize].get_coin_x() as f64 && player.get_x() + 3.0 < (platforms[i as usize].get_coin_x() + 24) as f64 && player.get_y() + player.get_height() as f64 - 3.0 > platforms[i as usize].get_coin_y() as f64 && player.get_y() + 3.0 < (platforms[i as usize].get_coin_y() + 24) as f64 {
            add_score(1, score_int, highscore_int, score, highscore);
            platforms[i as usize].set_has_coin(false);
            *play_coin_fx = true;
        }
        if player.get_x() + 1.0 < platforms[i as usize].get_x() + platforms[i as usize].get_width() as f64 && player.get_x() + player.get_width() as f64 > platforms[i as usize].get_x() && player.get_y() + player.get_height() as f64 >= platforms[i as usize].get_y() && player.get_y() < platforms[i as usize].get_y() + platforms[i as usize].get_height() as f64 {
            if player.get_y() > platforms[i as usize].get_y() + platforms[i as usize].get_height() as f64/2.0 {
                player.set_velocity(player.get_velocity().x as f64, 5.0);
            }
            else if (player.get_y() + player.get_height() as f64) < platforms[i as usize].get_y() + platforms[i as usize].get_height() as f64 {
                on_platform = true;
                player.set_y((platforms[i as usize].get_y() - (player.get_height() as f64)) as i32);
                player.set_y((player.get_y()+1.0) as i32);
            }
        }
        i += 1;
    }
    player.set_on_platform(on_platform);
}

fn main() -> () {
    println!("Initializing game...");

    //region "global" variables
    let mut i: i32 = 0;

    //region platform setup
    let mut platforms: [Platform; 4] = [platform_init(), platform_init(), platform_init(), platform_init()];
    while (i as usize) < platforms.len() {
        platforms[i as usize].create(i);
        i += 1;
    }
    i=0;
    //endregion

    //region player setup
    let mut player: Player = player_init();
    player.create(platforms.clone()[0].get_x() + (platforms[0].clone().get_width()/2 - 26/2) as f64, platforms[0].clone().get_y() - player.get_height() as f64, 26, 32);
    //endregion

    //region score setup
    let mut score_int: i32 = 0;
    let mut highscore_int: i32;
    unsafe {
        highscore_int = LoadStorageValue(0);
    }
    let mut score: String = "".to_string();
    let mut highscore: String = "".to_string();
    //endregion

    //region logic states
    let mut title_screen: bool = true;
    let mut play_coin_fx: bool = false;
    //endregion
    //endregion

    //region highscore stuff
    reset_score(&mut score_int, &mut highscore_int, &mut score);
    highscore = format!("BEST: {}", highscore_int);
    //endregion

    //region variables
    let mut mouse_down_x: i32 = 0;
    let mut mouse_down_y: i32 = 0;
    let mut lava_y: f64 = 0.0;
    let mut timer: f64 = 0.0;
    let mut splash_timer: f64 = 0.0;
    let mut first_time: bool = true;
    let mut played_splash: bool = false;
    let mut played_select: bool = false;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Terri-Fried")
        .build();
    //couldn't be bothered to find the correct function
    let egg: Image = Image::load_image("resources/sprites/egg.png").expect("could not load image");;
    rl.set_window_icon(egg);
    //endregion

    //region audio setup
    let mut audio = raylib::core::audio::RaylibAudio::init_audio_device();
    audio.set_master_volume(0.3);
    //endregion

    //region asset loading
    //region textures
    let player_sprite: Texture2D = rl.load_texture(&thread, "./resources/sprites/egg.png").unwrap();
    let lava_sprite: Texture2D = rl.load_texture(&thread, "./resources/sprites/lava.png").unwrap();
    let platform_sprite: Texture2D = rl.load_texture(&thread, "./resources/sprites/platform.png").unwrap();
    let coin_sprite: Texture2D = rl.load_texture(&thread, "./resources/sprites/coin.png").unwrap();
    let scorebox_sprite: Texture2D = rl.load_texture(&thread, "./resources/sprites/scorebox.png").unwrap();
    let logo: Texture2D = rl.load_texture(&thread, "./resources/sprites/logo.png").unwrap();
    let splashegg_sprite: Texture2D = rl.load_texture(&thread, "./resources/sprites/splash_egg.png").unwrap();
    //endregion

    //region sounds
    let fx_launch: Sound = Sound::load_sound("./resources/audio/launch.wav").unwrap();
    let fx_click: Sound = Sound::load_sound("./resources/audio/click.wav").unwrap();
    let fx_death: Sound = Sound::load_sound("./resources/audio/die.wav").unwrap();
    let fx_coin: Sound = Sound::load_sound("./resources/audio/coin.wav").unwrap();
    let fx_splash: Sound = Sound::load_sound("./resources/audio/splash.wav").unwrap();
    let fx_select: Sound = Sound::load_sound("./resources/audio/select.wav").unwrap();
    //endregion

    //region font
    let font: Font = rl.load_font_ex(&thread, "./resources/font/epicfont.otf", 64, FontLoadEx::Default(0)).unwrap();
    //endregion
    //endregion

    //region game logic
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if title_screen {
            if splash_timer > 120.0 {
                if !played_select {
                    audio.play_sound(&fx_select);
                    played_select = true;
                }
                let mut dhandle = rl.begin_drawing(&thread);
                dhandle.clear_background(raylib::color::Color::color_from_normalized(Vector4::new(0.933, 0.894, 0.882, 1.0)));
                dhandle.draw_texture(&logo, SCREEN_WIDTH/2 - 200, SCREEN_HEIGHT/2 - 45 - 30, Color::WHITE);
                dhandle.draw_text_ex(&font, &highscore, Vector2::new((SCREEN_WIDTH / 2 - 37) as f32, (SCREEN_HEIGHT / 2 + 10) as f32), 32 as f32, 0 as f32, Color::BLACK);
                dhandle.draw_text_ex(&font, "CLICK ANYWHERE TO BEGIN", Vector2::new((SCREEN_WIDTH / 2 - 134) as f32, (SCREEN_HEIGHT / 2 + 50) as f32), 32 as f32, 0 as f32, raylib::color::Color::color_from_normalized(Vector4::new(0.698, 0.588, 0.49, 0.4)));
                if dhandle.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
                   audio.play_sound(&fx_select);
                    title_screen = false;
                    mouse_down_x = dhandle.get_mouse_x();
                    mouse_down_y = dhandle.get_mouse_y();
                }
            }
            else {
                if !played_splash {
                    audio.play_sound(&fx_splash);
                    played_splash = true;
                }
                let mut dhandle = rl.begin_drawing(&thread);
                dhandle.clear_background(raylib::color::Color::color_from_normalized(Vector4::new(0.933, 0.894, 0.882, 1.0)));
                dhandle.draw_text_ex(&font, "POLYMARS", Vector2::new((SCREEN_WIDTH / 2 - 54) as f32, (SCREEN_HEIGHT / 2 + 3) as f32), 32 as f32, 0 as f32, raylib::color::Color::color_from_normalized(Vector4::new(0.835, 0.502, 0.353, 1.0)));
                dhandle.draw_text_ex(&font, "and DJ::Oetzi", Vector2::new((SCREEN_WIDTH / 2 - 50) as f32, (SCREEN_HEIGHT / 2 + 30) as f32), 24 as f32, 0 as f32, raylib::color::Color::color_from_normalized(Vector4::new(0.835, 0.502, 0.353, 1.0)));
                dhandle.draw_texture(&splashegg_sprite, SCREEN_WIDTH/2 - 16, SCREEN_HEIGHT/2 - 16 - 23, Color::WHITE);
                splash_timer += 1.0;
            }
        }
        else {
            if play_coin_fx {
                audio.play_sound(&fx_coin);
                play_coin_fx = false;
            }

            if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) && player.is_on_ground() {
                audio.play_sound(&fx_click);
                mouse_down_x = rl.get_mouse_x();
                mouse_down_y = rl.get_mouse_y();
            }

            if rl.is_mouse_button_released(MOUSE_LEFT_BUTTON) && player.is_on_ground() {
                if first_time {
                    first_time = false;
                }
                else {
                    audio.play_sound(&fx_launch);
                    if player.is_on_platform() {
                        player.set_y((player.get_y().clone() - 1.0) as i32);
                    }

                    let velocity_x = rl.get_mouse_x() - mouse_down_x;
                    let velocity_y = rl.get_mouse_y() - mouse_down_y;

                    player.set_velocity(velocity_x as f64 * 0.1, velocity_y as f64 * 0.1);
                }
            }
            check_player_collision(&mut platforms, &mut player, &mut score_int, &mut highscore_int, &mut score, &mut highscore, &mut play_coin_fx);
            player.update_position();

            if player.get_y() > SCREEN_HEIGHT as f64 {
                audio.play_sound(&fx_death);
                reset_game(&mut platforms, &mut player, &mut score_int, &mut highscore_int, &mut score);
            }

            i = 0;
            while i<4 {
                platforms[i as usize].update_position();
                i += 1;
            }
            lava_y = SCREEN_HEIGHT as f64 - 43.0 - (timer.sin() * 5.0);
            timer += 0.05;
            let mut dhandle = rl.begin_drawing(&thread);
            dhandle.clear_background(raylib::color::Color::color_from_normalized(Vector4::new(0.933, 0.894, 0.882, 1.0)));
            if dhandle.is_mouse_button_down(MOUSE_LEFT_BUTTON.clone()) && player.is_on_ground() {
                dhandle.draw_line_ex(Vector2::new((mouse_down_x + (player.get_x() - (mouse_down_x as f64) + (player.get_width() / 2) as f64) as i32) as f32, ((mouse_down_y + (player.get_y() - mouse_down_y as f64) as i32 + (player.get_height() / 2)) as f32)), Vector2::new(((dhandle.get_mouse_x() as f64 + (player.get_x() - mouse_down_x as f64) + (player.get_width() / 2) as f64) as f32), ((dhandle.get_mouse_y() as f64 + (player.get_y() - mouse_down_y as f64) + (player.get_height() / 2) as f64) as f32)), 3.0, color::Color::color_from_normalized(Vector4::new(0.906, 0.847, 0.788, 1.0)));
            }
            i=0;
            while i < 4 {
                dhandle.draw_texture(&platform_sprite, platforms[i as usize].get_x() as i32, platforms[i as usize].get_y() as i32, color::Color::color_from_normalized(Vector4::new(0.698, 0.588, 0.49, 1.0)));
                if platforms[i as usize].get_has_coin() {
                    dhandle.draw_texture(&coin_sprite, platforms[i as usize].get_coin_x(), platforms[i as usize].get_coin_y(), Color::WHITE);
                }
                i += 1;
            }
            dhandle.draw_texture(&player_sprite, player.get_x() as i32, player.get_y() as i32, Color::WHITE);
            dhandle.draw_texture(&lava_sprite, 0, lava_y as i32, Color::WHITE);
            dhandle.draw_texture(&scorebox_sprite, 17, 17, Color::WHITE);
            dhandle.draw_text_ex(&font, &score, Vector2::new(28.0, 20.0), 64.0, 0.0, Color::BLACK);
            dhandle.draw_text_ex(&font, &highscore, Vector2::new(17.0, 90.0), 32.0, 0.0, Color::BLACK);
        }
    }
    //endregion
}
