// Copyright (c) 2017-2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

use std::collections::HashMap;

///
#[derive(Debug, Serialize, Deserialize)]
enum MiscOption {
    UseSyslog,
    Color,
    TotalDownload,
    CheckSpace,
    VerbosePkgLists
}

/// SigLevel determines the level at which a package signature
/// is tested from an external source.
#[derive(Debug, Serialize, Deserialize)]
enum SigLevel {
    Required,
    DatabaseOptional,
    Optional
}

/// Config is a configuration struct which is loaded
/// like pacman.conf (default path: `/etc/pacman.conf`).
#[derive(Debug, Serialize, Deserialize)]
pub struct Config<'a> {
    #[serde(default = "default_root_dir")]
    root_dir: &'a str,
    #[serde(default = "default_db_path")]
    db_path: &'a str,
    #[serde(default = "default_cache_dir")]
    cache_dir: &'a str,
    #[serde(default = "default_log_file")]
    log_file: &'a str,
    #[serde(default = "default_gpg_dir")]
    gpg_dir: &'a str,
    #[serde(default = "default_hook_dir")]
    hook_dir: &'a str,
    #[serde(default = "default_hold_pkg")]
    hold_pkg: Vec<&'a str>,
    #[serde(default = "default_clean_method")]
    clean_method: &'a str,
    #[serde(default = "default_delta")]
    use_delta: &'a str,
    #[serde(default = "default_architecture")]
    architecture: &'a str,
    #[serde(default)]
    ignore_pkg: Vec<&'a str>,
    #[serde(default)]
    ignore_group: Vec<&'a str>,
    #[serde(default)]
    no_upgrade: Vec<&'a str>,
    #[serde(default)]
    no_extract: Vec<&'a str>,
    #[serde(default = "default_misc")]
    misc: Vec<MiscOption>,
    #[serde(default = "default_siglevel")]
    sig_level: Vec<SigLevel>,
    #[serde(default = "default_local_file_siglevel")]
    local_file_sig_level: Vec<SigLevel>,
    #[serde(default = "default_remote_file_sig_level")]
    remote_file_sig_level: Vec<SigLevel>,
    //
    repositories: HashMap<&'a str, HashMap<&'a str, &'a str>>
}

fn default_root_dir<'a>() -> &'a str {
    "/"
}

fn default_db_path<'a>() -> &'a str {
    "/var/lib/pacman"
}

fn default_cache_dir<'a>() -> &'a str {
    "/var/cache/pacman/pkg"
}

fn default_log_file<'a>() -> &'a str {
    "/var/log/pacng.log"
}

fn default_gpg_dir<'a>() -> &'a str {
    "/etc/pacman.d/gnupg/"
}

fn default_hook_dir<'a>() -> &'a str {
    "/etc/pacman.d/hooks/"
}

fn default_hold_pkg<'a>() -> Vec<&'a str> {
    vec!["pacman", "glibc"]
}

fn default_clean_method<'a>() -> &'a str {
    "KeepInstalled"
}

fn default_delta<'a>() -> &'a str {
    "0.7"
}

fn default_architecture<'a>() -> &'a str {
    "auto"
}

fn default_misc() -> Vec<MiscOption> {
    vec![
        MiscOption::Color,
        MiscOption::TotalDownload,
        MiscOption::CheckSpace
    ]
}

fn default_siglevel() -> Vec<SigLevel> {
    vec![
        SigLevel::Required, SigLevel::DatabaseOptional
    ]
}

fn default_local_file_siglevel() -> Vec<SigLevel> {
    vec![
        SigLevel::Optional
    ]
}

fn default_remote_file_sig_level() -> Vec<SigLevel> {
    vec![]
}

// impl<'a> Config<'a> {
//     fn new() -> Config<'a> {
//         Config {}
//     }
// }
