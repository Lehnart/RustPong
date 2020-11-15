use sdl2::mixer::{Chunk, Channel};
use engine::audio::init_audio;
use crate::logic::Logic;

pub const CHANNEL_COUNT : i32 = 6;

pub struct Audio {
    explosion: Chunk,
    forward: Chunk,
    turning: Chunk,
    shoot: Chunk,

    left_tank_move_channel: Channel,
    right_tank_move_channel: Channel,
    left_tank_turn_channel: Channel,
    right_tank_turn_channel: Channel,
    left_tank_fire_channel: Channel,
    right_tank_fire_channel: Channel,
}

impl Audio {
    pub fn new() -> Audio {
        init_audio(CHANNEL_COUNT);

        Audio {
            explosion: sdl2::mixer::Chunk::from_file("res/explosion.wav").unwrap(),
            forward: sdl2::mixer::Chunk::from_file("res/forward.wav").unwrap(),
            turning: sdl2::mixer::Chunk::from_file("res/turning.wav").unwrap(),
            shoot: sdl2::mixer::Chunk::from_file("res/shoot.wav").unwrap(),

            left_tank_move_channel : sdl2::mixer::Channel(0),
            right_tank_move_channel : sdl2::mixer::Channel(1),
            left_tank_turn_channel : sdl2::mixer::Channel(2),
            right_tank_turn_channel : sdl2::mixer::Channel(3),
            left_tank_fire_channel: sdl2::mixer::Channel(4),
            right_tank_fire_channel: sdl2::mixer::Channel(5)
        }
    }

    pub fn update(&mut self, logic : &Logic){

        if logic.left_tank.is_moving() {
            if !self.left_tank_move_channel.is_playing() {
                self.left_tank_move_channel.play(&self.forward, -1);
            }
        }
        else {
            if self.left_tank_move_channel.is_playing(){
                self.left_tank_move_channel.halt();
            }
        }

        if logic.left_tank.is_turning() {
            if !self.left_tank_turn_channel.is_playing() {
                self.left_tank_turn_channel.play(&self.turning, -1);
            }
        }
        else {
            if self.left_tank_turn_channel.is_playing(){
                self.left_tank_turn_channel.halt();
            }
        }
    }

    pub fn play_left_explosion(&self) {
        self.left_tank_fire_channel.play(&self.explosion,0);
    }

    pub fn play_right_explosion(&self) {
        self.right_tank_fire_channel.play(&self.explosion,0);
    }
    pub fn play_left_shoot(&self) {
        self.left_tank_fire_channel.play(&self.shoot,0);
    }
    pub fn play_right_shoot(&self) {
        self.right_tank_fire_channel.play(&self.shoot,0);
    }
}
