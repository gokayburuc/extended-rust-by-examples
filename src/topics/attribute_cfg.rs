// This function only gets compiled if the target os is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// this function only gets compiled if the target os is not linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

pub fn execute_cfg_attribute() {
    are_you_on_linux();

    println!("Are you sure?");

    // cfg! expression gives a boolean result
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
