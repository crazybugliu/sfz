// Copyright (c) 2018 Weihang Lo
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::path::PathBuf;
use std::net::SocketAddr;

use clap::ArgMatches;

#[derive(Debug, Clone)]
pub struct Args {
    pub address: String,
    pub port: u16,
    pub cache: u32,
    pub cors: bool,
    pub compress: bool,
    pub path: PathBuf,
    pub all: bool,
    pub ignore: bool,
    pub follow_links: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_owned(),
            port: 8888,
            cache: 0,
            compress: true,
            cors: false,
            path: env::current_dir().unwrap_or_default(),
            all: false,
            ignore: true,
            follow_links: false,
        }
    }
}

impl Args {
    /// Parse arguments.
    pub fn parse(matches: ArgMatches) -> Args {
        let address = matches.value_of("address")
            .unwrap_or_default()
            .to_owned();
        let port = value_t!(matches.value_of("port"), u16)
            .unwrap_or_default();
        let cache = value_t!(matches.value_of("cache"), u32)
            .unwrap_or_default();
        let cors = matches.is_present("cors");
        let path = {
            let path = matches.value_of("path").unwrap_or_default();
            let path = PathBuf::from(path);
            if path.is_absolute() {
                path
            } else {
                env::current_dir().unwrap_or_default()
                    .join(path)
                    .canonicalize().unwrap()
            }
        };
        let compress = !matches.is_present("unzipped");
        let all = matches.is_present("all");
        let ignore = !matches.is_present("no-ignore");
        let follow_links = matches.is_present("follow-links");

        Args {
            address,
            port,
            cache,
            cors,
            path,
            compress,
            all,
            ignore,
            follow_links,
            ..Default::default()
        }
    }

    /// Construct socket address from arguments.
    pub fn address(&self) -> SocketAddr {
        format!("{}:{}", self.address, self.port).parse().unwrap()
    }
}
