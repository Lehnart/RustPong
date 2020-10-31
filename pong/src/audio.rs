use sdl2::mixer::Chunk;

pub struct Audio {
    wall_bounce: Chunk,
    racket_bounce: Chunk,
    lose: Chunk,
}

impl Audio {
    pub fn new() -> Audio {
        Audio {
            wall_bounce: sdl2::mixer::Chunk::from_file("res/wall.wav").unwrap(),
            racket_bounce: sdl2::mixer::Chunk::from_file("res/racket.wav").unwrap(),
            lose: sdl2::mixer::Chunk::from_file("res/lose.wav").unwrap(),
        }
    }

    pub fn play_wall_bounce(&self) {
        sdl2::mixer::Channel::all().play(&self.wall_bounce, 0).unwrap();
    }
    pub fn play_racket_bounce(&self) {
        sdl2::mixer::Channel::all().play(&self.racket_bounce, 0).unwrap();
    }
    pub fn play_lose(&self) {
        sdl2::mixer::Channel::all().play(&self.lose, 0).unwrap();
    }
}
