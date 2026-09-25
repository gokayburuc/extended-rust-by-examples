#[allow(dead_code)]
pub fn execute_assert() {
    let x = 5;
    let y = 3;

    assert_eq!(x, y, "{} = {} ", x, y);

    // assert
    let has_licence = true;
    assert!(has_licence, "Has No licence!");

    // TODO: assert_eq!
    // TODO: assert_matches!
    // TODO: assert_ne!
}
