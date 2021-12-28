use crate::player::{InputMask, Input};

#[test]
fn test_new_inputmask() {
    let inputmask = InputMask::new();

    assert!(!inputmask.has_mask(Input::Up));
    assert!(!inputmask.has_mask(Input::Down));
    assert!(!inputmask.has_mask(Input::Left));
    assert!(!inputmask.has_mask(Input::Right));
}

#[test]
fn test_inputmask_mask() {
    let mut inputmask = InputMask::new();

    assert!(!inputmask.has_mask(Input::Up));

    inputmask.add_mask(Input::Up);

    assert!(inputmask.has_mask(Input::Up));
    assert!(!inputmask.has_mask(Input::Down));
}
