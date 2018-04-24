// Copyright (c) 2017-2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC
#![allow(unused_imports)]
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;
use std::ffi::OsString;

extern crate pacng;

use pacng::PacNG;
use pacng::config::Config;
use pacng::error;
use pacng::mirrorlist::Mirrorlist;

#[macro_use] extern crate clap;
#[macro_use] extern crate slog;
extern crate chrono;
extern crate runas;
extern crate slog_async;
extern crate slog_term;
extern crate toml;
extern crate users;

use clap::{App, AppSettings, Arg, SubCommand};
use slog::{Logger, Drain};

// Build the clap App argparser
fn build_args<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .about("A drop-in alternative for the `pacman` package manager for ArchLinux.")
        .author(crate_authors!())
        .settings(&[
            AppSettings::ColorAuto,
            AppSettings::WaitOnError
        ])
        // Top level arguments
        .args(&[
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Enable debugging output"),
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enable verbose logging output"),
        ])
        // Legacy pacman flags
        // To be removed by v2.0.x
        .args(&[
            Arg::with_name("database")
                .short("D")
                .long("database")
                .help("Legacy database flag"),
            Arg::with_name("files")
                .short("F")
                .long("files")
                .help("Legacy files flag"),
            Arg::with_name("query")
                .short("Q")
                .long("query")
                .help("Legacy query flag"),
            Arg::with_name("Remove")
                .short("R")
                .long("remove")
                .help("Legacy remove flag"),
            Arg::with_name("sync")
                .short("S")
                .long("sync")
                .help("Legacy sync flag"),
            Arg::with_name("deptest")
                .short("T")
                .long("deptest")
                .help("Legacy deptest flag"),
            Arg::with_name("upgrade")
                .short("U")
                .long("upgrade")
                .help("Legacy upgrade flag"),
            Arg::with_name("mirrorlist")
                .short("M")
                .long("mirrorlist")
                .long_help("Update the pacman mirrorlist")
                .conflicts_with_all(&["upgrade", "deptest", "sync", "remove", "query", "files", "database"])
        ])
        // Top level subcommands
        .subcommands(vec![
            SubCommand::with_name("database")
                .about("Pacman Database operations"),
            SubCommand::with_name("files")
                .about("Pacman Files operations"),
            SubCommand::with_name("query")
                .about("Pacman Query operations"),
            SubCommand::with_name("remove")
                .about("Pacman Remove operations"),
            SubCommand::with_name("sync")
                .about("Pacman Sync operations"),
            SubCommand::with_name("deptest")
                .about("Pacman Dependency Testing operations"),
            SubCommand::with_name("upgrade")
                .about("Pacman Upgrade operations"),
            SubCommand::with_name("mirrorlist")
                .about("Update the pacman mirrorlist")
                .version("0.1.0")
                .args(&[
                    Arg::with_name("timeout")
                        .long("connection-timeout"),
                    Arg::with_name("list_countries")
                        .long("list-countries"),
                    Arg::with_name("cache_timeout")
                        .long("cache_timeout"),
                    Arg::with_name("save")
                        .long("save")
                        .short("o"),
                    Arg::with_name("sort_by")
                        .long("sort"),
                    Arg::with_name("info")
                        .long("info"),
                ])
        ])
}

/// Main executable function.
fn main() {

    // Set up the terminal logger
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let _log = Logger::root(drain, o!());

    let parser = build_args().get_matches();

    let mut path = Path::new("/etc/pacng.toml");

    if parser.is_present("config") {

    }

    // let mut fd = File::open(path).expect("Could not open configuration file");
    // let mut buf = Vec::new();
    // fd.read_to_end(&mut buf).expect("Failed to properly read configuration file");
    //
    // let cfg: Config = toml::from_slice(&buf).expect("Failed to deserialize the configuration file");

    // Initialize the PacNG struct
    // let pcng: PacNG = PacNG::new(&cfg);

    // Check for the mirrorlist command
    // if parser.is_present("mirrorlist") {
    //     let ml = Mirrorlist::new();
    //     match ml::update() {
    //         Ok(()) => info!(_log, "Mirrorlist successfully updated"),
    //         Err(e) => error!(_log, "Mirrorlist update failed: {:?}", e)
    //     }
    // }
}
