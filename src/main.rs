mod bin_res;

use std::time::Duration;

use log::debug;

use crate::bin_res::lib_wrapper::*;

fn main() {
    env_logger::init();

    lib_init().unwrap();

    let hook = lib_set_hook().unwrap();

    ctrlc::set_handler(move || {
        lib_unset_hook(hook).unwrap();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    lib_block_on_msg().unwrap();

    lib_unset_hook(hook).unwrap();
}
