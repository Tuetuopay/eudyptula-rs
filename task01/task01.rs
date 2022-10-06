//! Simple hello world module

use kernel::prelude::*;

module! {
    type: Task01,
    name: "task01",
    author: "Tuetuopay",
    description: "Eudyptula task 01",
    license: "GPL",
}

struct Task01;

impl kernel::Module for Task01 {
    fn init(_name: &CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World!\n");
        Ok(Task01)
    }
}

impl Drop for Task01 {
    fn drop(&mut self) {
        pr_info!("cya!\n");
    }
}
