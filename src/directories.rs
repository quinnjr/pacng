// Copyright (c) 2017-2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

#[allow(unused_imports)]
use std::io::prelude::*;
use std::env;
use std::fs::DirBuilder;
use std::path::PathBuf;
use std::io::Error;

pub fn temp_dir() -> Result<PathBuf, Error> {
    let mut dir = env::temp_dir();
    dir.push("pacng");
    if !dir.is_dir() {
        match DirBuilder::new().recursive(true).create(&dir) {
            Ok(_) => Ok(dir),
            Err(e) => Err(e)
        }
    } else {
        Ok(dir)
    }
}
