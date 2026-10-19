pub fn execute_assert() {
    let x: i32 = 5;
    let y: i32 = 3;

    assert_eq!(x, y, "{} = {} ", x, y);

    // assert
    let has_licence: bool = true;
    assert!(has_licence, "Has No licence!");

    // TODO: assert_eq!
    // TODO: assert_matches!
    // TODO: assert_ne!
}
