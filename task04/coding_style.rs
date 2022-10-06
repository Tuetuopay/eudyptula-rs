//! Coding style exercise

use core::time::Duration;
use kernel::{delay::sleep, prelude::*};

module! {
    type: CodingStyle,
    name: "coding_style",
    license: "GPL",
}

struct CodingStyle;

impl kernel::Module for CodingStyle {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let x = 10;
        let x = do_work(&x, x);

        // The kernel crate does not let us craft arbitrary errnos. So we log the code and carry
        // on.
        pr_err!("{x}\n");
        Ok(CodingStyle)
    }
}

impl Drop for CodingStyle {
    fn drop(&mut self) {}
}

fn do_work(my_int: &i32, _retval: i32) -> i32 {
    let y = *my_int;

    for _ in 0..*my_int {
        sleep(Duration::from_micros(10));
    }
    let x = *my_int;

    if y < 10 {
        // That was a long sleep, tell userspace about it.
        pr_debug!("We slept a long time!\n");
    }

    x * y
}
