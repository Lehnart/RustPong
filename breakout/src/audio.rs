use sdl2::mixer::Chunk;

/// Structure containing all the sounds that will be played during the game.
pub struct Audio {
    wall_bounce: Chunk,
    racket_bounce: Chunk,
    block_bounce: Chunk,
}

impl Audio {
    pub fn new() -> Audio {
        Audio {
            wall_bounce: sdl2::mixer::Chunk::from_file("res/wall.wav").unwrap(),
            racket_bounce: sdl2::mixer::Chunk::from_file("res/racket.wav").unwrap(),
            block_bounce: sdl2::mixer::Chunk::from_file("res/block.wav").unwrap(),
        }
    }

    pub fn play_wall_bounce(&self) {
        sdl2::mixer::Channel::all().play(&self.wall_bounce, 0).unwrap();
    }
    pub fn play_racket_bounce(&self) {
        sdl2::mixer::Channel::all().play(&self.racket_bounce, 0).unwrap();
    }
    pub fn play_block_bounce(&self, loops: i32) {
        sdl2::mixer::Channel::all().play(&self.block_bounce, loops).unwrap();
    }
}
