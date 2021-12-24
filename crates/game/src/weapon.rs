use core::fmt::Debug;

pub type Weapon = Box<dyn WeaponTrait>;

pub trait WeaponTrait: Debug {
    fn attack(&self);
    fn throw(&self);
    fn get_owner(&self);
    fn get_team(&self);
}
