// Copyright (c) 2017-2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

use error;

// use std::fs;

fn change_install_reason(targets: &Vec<String>) -> Result<bool, error::Error> {
    Ok(true)
}

fn check_missing_deps(pkglist: &Vec<String>) -> Result<bool, error::Error> {
    Ok(true)
}

fn check_local_files() -> Result<String, error::Error> {
    Ok("hurf".to_string())
}

fn check_local_package_conflicts(pkglist: &Vec<String>) -> Result<bool, error::Error> {
    Ok(true)
}

///
struct Database {
}

struct Package {
}
