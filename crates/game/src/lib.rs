mod map;
mod block;

#[cfg(test)]
mod tests {

    #[test]
    fn allow_basic_map() {
        let woodblocks: u128 = 0b0101;
        let woodplanks: u128 = 0b1010;

        let result = map::Map::from_bits(woodblocks, woodplanks, 0, 0, 0, 0);

        assert!(result.is_ok());
    }

    #[test]
    fn forbid_basic_map() {
        let woodblocks: u128 = 0b0101;
        let woodplanks: u128 = 0b1110;

        let result = map::Map::from_bits(woodblocks, woodplanks, 0, 0, 0, 0);

        assert!(result.is_err());
    }

    #[test]
    fn disallow_empty_map() {
        let result = map::Map::from_bits(0, 0, 0, 0, 0, 0);
        assert!(result.is_err());
    }

    #[test]
    fn disallow_full_overlap_map() {
        let m = u128::MAX;
        let result = map::Map::from_bits(m, m, m, m, m, m);
        assert!(result.is_err());
    }
}
