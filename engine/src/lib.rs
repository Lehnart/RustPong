pub mod physics;
pub mod geometry;
pub mod random;
pub mod collide;
pub mod audio;
pub mod graphics;

pub mod engine {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
