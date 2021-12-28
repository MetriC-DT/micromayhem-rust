use crate::block::{get_block_friction, BlockType};

#[test]
fn test_get_block_friction() {
    assert_eq!(get_block_friction(BlockType::GrassBlock), 1.0);
    assert_eq!(get_block_friction(BlockType::IceBlock), 0.5);
}
