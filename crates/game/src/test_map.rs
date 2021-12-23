use crate::map;

#[test]
fn allow_basic_map_init() {
    let woodblocks: u128 = 0b0101;
    let woodplanks: u128 = 0b1010;

    let data = [woodblocks, woodplanks, 0, 0, 0, 0].into();
    let result = map::Map::from_bits(data);

    assert!(result.is_ok());
}

#[test]
fn allow_basic_map_2_init() {
    let a: u128 = 0b000000001;
    let b: u128 = 0b000000010;
    let c: u128 = 0b000100000;
    let d: u128 = 0b001000000;
    let e: u128 = 0b100000000;
    let f: u128 = 0b010000000;

    let data = [a,b,c,d,e,f].into();

    let result = map::Map::from_bits(data);
    assert!(result.is_ok());
}

#[test]
fn allow_basic_map_3_init() {
    let data = [0, 0, 0, 0, 0, 1].into();

    let result = map::Map::from_bits(data);
    assert!(result.is_ok());
}

#[test]
fn forbid_basic_map_init() {
    let woodblocks: u128 = 0b0101;
    let woodplanks: u128 = 0b1110;

    let data = [woodblocks, woodplanks, 0, 0, 0, 0].into();
    let result = map::Map::from_bits(data);

    assert!(result.is_err());
}

#[test]
fn disallow_full_overlap_map_init() {
    let m = u128::MAX;
    let data = [m, m, m, m, m, m].into();
    let result = map::Map::from_bits(data);
    assert!(result.is_err());
}
