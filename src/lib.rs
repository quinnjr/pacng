// Copyright (c) 2017-2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC
// It is dangerous to go alone! Take this. ⚔
#![cfg(target_os = "linux")]

#![allow(unused_variables)] // Remove in release
#![allow(dead_code)] // Remove in release


#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate regex;
extern crate serde_json;
extern crate serde;
extern crate reqwest;

pub mod config;
pub mod database;
pub mod deptest;
pub mod error;
pub mod files;
pub mod query;
pub mod remove;
pub mod sync;
pub mod upgrade;
pub mod mirrorlist;
pub mod directories;

use config::Config;

/// PacNG ...
#[derive(Debug)]
pub struct PacNG<'a> {
    config: &'a Config<'a>
}

/// Implementation of the functions on PacNG
impl<'a> PacNG<'a> {

    /// Initilize a new PacNG.
    pub fn new(config: &'a Config) -> Self {
        // let path = ::Path::new("/etc/pacman.conf");
        // let mut fd = ::fs::File::open(&path).expect("Unable to locate pacman configuration");
        // let mut data = Vec::new();
        // fd.read_to_end(data).expect("Unable to read file to vec");
        // let conf = ::serde_ini::from_read(&fd);
        PacNG {
            config: &config
        }
    }

    /// Load the configuration from pacman.
    /// Allows for the specifying of an alternative
    /// configuration file.
    fn load_config(path: String) {
    }
}
