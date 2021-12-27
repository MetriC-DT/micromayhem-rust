use core::fmt::Debug;

use glam::Vec2;

use crate::player::Player;

pub trait Weapon: Debug {
    fn attack(&self);
    fn throw(&self);
    fn get_owner(&self) -> &Player;
    fn get_team(&self) -> usize;
    fn clone_weapon(&self) -> Box<dyn Weapon>;
    fn get_weight(&self) -> f32;
}

pub trait Bullet: Debug {
    fn get_velocity(&self) -> Vec2;
    fn get_weight(&self) -> Vec2;
    fn get_position(&self) -> Vec2;
    fn update(&mut self);
    fn clone_bullet(&self) -> Box<dyn Bullet>;
}

// different types of guns and bullets below.
#[derive(Debug, Clone)]
pub struct BasicGun {
}

impl Weapon for BasicGun {
    fn attack(&self) {
        todo!()
    }

    fn throw(&self) {
        todo!()
    }

    fn get_owner(&self) -> &Player {
        todo!()
    }

    fn get_team(&self) -> usize {
        todo!()
    }

    fn clone_weapon(&self) -> Box<dyn Weapon> {
        return Box::new(self.clone());
    }

    fn get_weight(&self) -> f32 {
        todo!()
    }
}
