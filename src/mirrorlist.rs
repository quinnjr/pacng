// Copyright (c) 2017-2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

#![allow(dead_code)]
#![allow(unused_imports)]

// Standard libraries
use std::io::prelude::*;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use std::str;
use std::time::{SystemTime, Duration};
#[allow(unused_imports)]
use std::mem;

// Internal libraries
use directories;

// Third-party libraries
use reqwest;
use regex::Regex;
use serde_json;
use serde_json::value::Value;

#[derive(Debug)]
pub struct Mirrorlist;

const ARCH_MIRRORS_API: &str = "https://www.archlinux.org/mirrors/status/json/";

impl Mirrorlist {
    pub fn new() -> Mirrorlist {
        Mirrorlist {}
    }

    pub fn update(&self) -> Result<(), Box<Error>> {

        let cache_file = File::create("/var/cache/pacng/mirrorlist").expect("Unable to open the cache file");

        if self.check_cache_age(&cache_file) {

        }

        let response: Response = reqwest::get(ARCH_MIRRORS_API)?
            .json()?;

        Ok(())

    }

    fn check_cache_age(&self, cache_file: &File) -> bool {
        let now = SystemTime::now();
        let metadata = cache_file.metadata().unwrap();
        let cache_file_age = now.duration_since(metadata.created().unwrap()).unwrap();
        // If the cache file is older than 2 minutes.
        if cache_file_age > Duration::new(120, 0) {
            return true;
        } else {
            return false;
        }
    }


}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    cutoff: usize,
    check_frequency: usize,
    num_checks: usize,
    last_check: String,
    version: usize,
    urls: Vec<Mirror>
}

#[derive(Debug, Serialize, Deserialize)]
struct Mirror {
    protocol: String,
    url: String,
    country: String,
    last_sync: Value, // String or Null
    delay: Value, // usize or Null
    isos: bool,
    score: Value, // f32 or Null
    completion_pct: Value, // f32 or null
    country_code: String,
    duration_stddev: Value, // f32 or null
    duration_avg: Value, // f32 or null
    details: String
}
