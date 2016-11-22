extern crate iron;
extern crate hoedown;
extern crate hyper;
extern crate webbrowser;
extern crate router;
extern crate staticfile;
extern crate clap;
extern crate linked_hash_map;

#[macro_use]
extern crate lazy_static;

use iron::prelude::*;

use std::path::Path;
use std::fs;

use router::Router;
use staticfile::Static;

use clap::{Arg, App};

#[macro_use]
mod macros;

mod views;
mod yaml_utils;
mod configuration;

fn main() {
    let matches = arg_matches();

    if check_data_directory() {
        println!("Listening on port 9000 ...");

        if no_browser(& matches) {
            println!("Open http://localhost:9000 to view your project")
        }

        Iron::new(make_router()).http("localhost:9000").unwrap();
    } else {
        println!("Failed to create data directory");
    }
}

fn arg_matches<'a>() -> clap::ArgMatches<'a> {
    App::new("cardboard")
        .version("v0.1")
        .author("Markus Kahl <machisuji@gmail.com>")
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .help("Starts cardboard without opening it in the browser.")
        )
        .get_matches()
}

fn make_router() -> Router {
    let mut router = Router::new();

    router.get("/", views::index, "index");
    router.get("*", file_handler(), "files");

    router
}

fn file_handler() -> Static {
    Static::new(Path::new("public/"))
}

fn check_data_directory() -> bool {
    let root = ".cardboard";
    let cards = root.to_string() + "/cards";

    (fs::create_dir(root).is_ok() && fs::create_dir(& cards).is_ok()) || Path::new(& cards).exists()
}

fn no_browser(matches: &clap::ArgMatches) -> bool {
    matches.occurrences_of("quiet") == 1 || !webbrowser::open("http://localhost:9000/").is_ok()
}
