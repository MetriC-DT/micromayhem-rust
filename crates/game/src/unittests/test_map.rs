mod tests {
    use std::path::Path;
    use crate::{block::BlockType, map::{Map, MapBlocksList, VERTICAL_BLOCKS}};
    use strum::EnumCount;

    #[test]
    fn allow_basic_map_1_init() {
        let a: i128 = 0b0101;
        let b: i128 = 0b1010;

        let mut data: [i128; BlockType::COUNT] = [0; BlockType::COUNT];
        data[0] = a;
        data[1] = b;
        let result = Map::from_mapblocks(data.into());

        assert!(result.is_ok());
    }

    #[test]
    fn allow_basic_map_2_init() {
        let a: i128 = 0b000000001;
        let b: i128 = 0b000100000;

        let mut data: [i128; BlockType::COUNT] = [0; BlockType::COUNT];
        data[0] = a;
        data[1] = b;

        let result = Map::from_mapblocks(data.into());
        assert!(result.is_ok());
    }

    #[test]
    fn allow_basic_map_3_init() {
        let data: [i128; BlockType::COUNT] = [0; BlockType::COUNT];
        let result = Map::from_mapblocks(data.into());
        assert!(result.is_ok());

        let mut data2: [i128; BlockType::COUNT] = [0; BlockType::COUNT];
        data2[0] = 1;

        let result2 = Map::from_mapblocks(data2.into());
        assert!(result2.is_ok());
    }

    #[test]
    fn forbid_basic_map_init() {
        let a: i128 = 0b0101;
        let b: i128 = 0b1110;

        let mut data: [i128; BlockType::COUNT] = [0; BlockType::COUNT];
        data[0] = a;
        data[1] = b;
        let result = Map::from_mapblocks(data.into());

        assert!(result.is_err());
    }

    #[test]
    fn disallow_full_overlap_map_init() {
        let m = -1;
        let data: [i128; BlockType::COUNT] = [m; BlockType::COUNT];
        let result = Map::from_mapblocks(data.into());
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_deserialize_map() {
        let f = "test_map";
        let path = Path::new(f);
        let mut data: [i128; BlockType::COUNT] = [0; BlockType::COUNT];
        data[0] = -1;
        let result = Map::from_mapblocks(data.into()).unwrap();

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

    #[test]
    fn test_display_map() {
        let data: MapBlocksList = [0; BlockType::COUNT];
        let matchstr = "0000000000000000\n0000000000000000\n0000000000000000\n0000000000000000\n0000000000000000\n0000000000000000\n0000000000000000\n0000000000000000\n";
        let m = Map::from_mapblocks(data.into()).unwrap();

        assert_eq!(m.get_all_occupied().to_string(), matchstr);
    }

    #[test]
    fn test_display_map_full() {
        let mut data: MapBlocksList = [0; BlockType::COUNT];
        data[0] = -1;
        let matchstr = "1111111111111111\n1111111111111111\n1111111111111111\n1111111111111111\n1111111111111111\n1111111111111111\n1111111111111111\n1111111111111111\n";
        let m = Map::from_mapblocks(data.into()).unwrap();
        assert_eq!(m.get_all_occupied().to_string(), matchstr);
    }

    #[test]
    fn test_to_blocktypes() {
        let grassbits: i128 = 0b000000001;
        let icebits: i128 = 0b000100000;
        let mut data: [i128; BlockType::COUNT] = [0; BlockType::COUNT];
        data[BlockType::GrassBlock as usize] = grassbits;
        data[BlockType::IceBlock as usize] = icebits;
        let map = Map::from_mapblocks(data.into()).unwrap();

        let (row, col) = (5, 0);
        let blocktype = map.to_blocktypes()[col * VERTICAL_BLOCKS + row].unwrap();
        assert_eq!(BlockType::IceBlock as usize, blocktype as usize);
    }

    #[test]
    fn test_first_row_below() {
        let grassbits: i128 = 0b000000001;
        let icebits: i128 = 0b000100000;
        let mut data: [i128; BlockType::COUNT] = [0; BlockType::COUNT];
        data[BlockType::GrassBlock as usize] = grassbits;
        data[BlockType::IceBlock as usize] = icebits;
        let map = Map::from_mapblocks(data.into()).unwrap();

        assert_eq!(map.first_row_below(1, 0), Some(5));
        assert_eq!(map.first_row_below(0, 0), Some(0));
        assert_eq!(map.first_row_below(2, 0), Some(5));
        assert_eq!(map.first_row_below(6, 0), None);
        assert_eq!(map.first_row_below(0, 1), None);
    }
}
