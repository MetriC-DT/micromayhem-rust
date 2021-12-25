use std::path::Path;

use crate::map::Map;
#[test]
fn allow_basic_map_init() {
    let a: u128 = 0b0101;
    let b: u128 = 0b1010;

    let data = [a, b, 0].into();
    let result = Map::default(data);

    assert!(result.is_ok());
}

#[test]
fn allow_basic_map_2_init() {
    let a: u128 = 0b000000001;
    let b: u128 = 0b000000010;
    let c: u128 = 0b000100000;

    let data = [a,b,c].into();

    let result = Map::default(data);
    assert!(result.is_ok());
}

#[test]
fn allow_basic_map_3_init() {
    let data = [0, 0, 1].into();

    let result = Map::default(data);
    assert!(result.is_ok());
}

#[test]
fn forbid_basic_map_init() {
    let a: u128 = 0b0101;
    let b: u128 = 0b1110;

    let data = [a,b,0].into();
    let result = Map::default(data);

    assert!(result.is_err());
}

#[test]
fn disallow_full_overlap_map_init() {
    let m = u128::MAX;
    let data = [m, m, m].into();
    let result = Map::default(data);
    assert!(result.is_err());
}

#[test]
fn test_serialize_deserialize_map() {
    let f = "test_map";
    let path = Path::new(f);
    let data = [u128::MAX, 0, 0].into();
    let result = Map::default(data).unwrap();

    let writesuccess = result.write_to_file(f);

    // verifies if function returned ok and a file was created.
    assert!(writesuccess.is_ok());
    assert!(path.exists());

    let map_from_file = Map::read_from_file(f).unwrap();

    // deletes the file
    std::fs::remove_file(path).unwrap();

    assert_eq!(map_from_file.to_string(), result.to_string());
}

#[test]
fn test_deserialize_fail() {
    let f = "nonexistant_file";
    assert!(Map::read_from_file(f).is_err());
}
