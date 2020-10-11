use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, Chunk};
use crate::logic::Logic;
use crate::collide::Collide;

pub(crate) struct Audio{
    wall_bounce : Chunk,
    racket_bounce : Chunk,
    lose : Chunk
}

impl Audio {
    pub fn new() -> Audio {

        let frequency = 44_100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;
        sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

        // Number of mixing channels available for sound effect `Chunk`s to play
        // simultaneously.
        sdl2::mixer::allocate_channels(4);

        Audio{
            wall_bounce: sdl2::mixer::Chunk::from_file("res/wall.wav").unwrap(),
            racket_bounce: sdl2::mixer::Chunk::from_file("res/racket.wav").unwrap(),
            lose: sdl2::mixer::Chunk::from_file("res/lose.wav").unwrap(),
        }

    }

    pub fn update(& self,logic : &Logic, collide: &Collide){
        if logic.is_point(){
            self.play_lose();
        }

        if collide.is_collide(){
            self.play_racket_bounce();
        }

        if logic.is_wall_collide(){
            self.play_wall_bounce();
        }

    }

    fn play_wall_bounce(&self){
        sdl2::mixer::Channel::all().play(&self.wall_bounce, 0).unwrap();
    }
    fn play_racket_bounce(&self){
        sdl2::mixer::Channel::all().play(&self.racket_bounce, 0).unwrap();
    }
    fn play_lose(&self){
        sdl2::mixer::Channel::all().play(&self.lose, 0).unwrap();
    }

}
