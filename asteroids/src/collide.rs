
use crate::logic::{Logic, Asteroid, Bullet};
use engine::geometry::AsRect;
use engine::collide::collide;

fn collide_shell_and_asteroids(asteroids : &mut Vec<Asteroid>, bullets : &mut Vec<Bullet>){
    for bullet in bullets{
        let bullet_rect = bullet.as_rect();
        for asteroid in &mut *asteroids {
            let asteroid_rect = asteroid.as_rect();
            match collide(&bullet_rect,&asteroid_rect) {
                None => (),
                Some(_rect) => {
                    bullet.destroy();
                    asteroid.destroy();
                }
            }
        }
    }
}

pub fn check_collision(logic: &mut Logic) {
    collide_shell_and_asteroids(&mut logic.asteroids.vec, &mut logic.spaceship.bullets);
}