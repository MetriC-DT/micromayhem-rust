pub type Weapon = Box<dyn WeaponTrait>;

pub trait WeaponTrait {
    fn attack(&self);
    fn throw(&self);
    fn get_owner(&self);
    fn get_team(&self);
}
