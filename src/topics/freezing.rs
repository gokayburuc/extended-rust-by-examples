#[allow(dead_code)]
pub fn execute_freezing() {
    let mut _mutable_integer = 1;
    {
        // WARN:  second _mutable_integer is immutable
        // let _mutable_integer = _mutable_integer;

        // WARN: this will give an error  : immutable variable error
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}
