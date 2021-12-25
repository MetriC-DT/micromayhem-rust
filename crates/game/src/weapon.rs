use core::fmt::Debug;

use crate::player::Player;

pub trait Weapon: Debug {
    fn attack(&self);
    fn throw(&self);
    fn get_owner(&self) -> &Player;
    fn get_team(&self) -> usize;
    fn clone_weapon(&self) -> Box<dyn Weapon>;
}

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
}
