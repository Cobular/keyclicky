mod bin_res;

use std::{io::BufReader, fs::File};

use log::debug;
use rodio::{OutputStream, Decoder, Source};

use crate::bin_res::lib_wrapper::*;

fn main() {
    env_logger::init();

    debug!("ee");
    
    let hook = lib_set_hook().unwrap();

    ctrlc::set_handler(move || {
        lib_unset_hook(hook).unwrap();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    lib_block_on_msg().unwrap();

    lib_unset_hook(hook).unwrap();
}