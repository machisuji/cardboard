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

mod git;
mod api;
mod views;
mod yaml_utils;
mod text_files;
mod configuration;

fn main() {
    let matches = arg_matches();

    if check_data_directory() {
        let repo = git::open_or_init(".cardboard");
        let state = repo.head().ok()
            .and_then( |state| {
                let msg = state.target()
                    .and_then(|oid| repo.find_commit(oid).ok())
                    .and_then(|mut commit| commit.summary().map(|s| s.to_string()))
                    .unwrap_or(String::from("no summary"));

                state.shorthand().map(|s| format!("{} ({})", s.to_string(), msg))
            })
            .unwrap_or(String::from("<new>"));

        if state == "<new>" {
            let sha = git::initial_commit(".", "initial commit", &repo);

            println!("Data at {:?}", sha);
        } else {
            println!("Data at {}", state);
        }

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

    router.post("/update_card", api::update_card, "update_card");

    router
}

fn file_handler() -> Static {
    Static::new(Path::new("public/"))
}

fn check_data_directory() -> bool {
    let root = ".cardboard";
    let cards = root.to_string() + "/cards";
    let default_data = fs::create_dir(root).is_ok();

    // populate newly created .cardboard directory with example data
    if default_data {
        println!("No data found. Initializing with example data.");
        copy_directory("public/example", ".cardboard");
    }

    !default_data || Path::new(& cards).exists()
}

/**
 * Copies the given directory to another location as far as possible.
 * Meaning that it will continue even if single files or subdirectories
 * cannot be copied.
 */
fn copy_directory(from_path: &str, to_path: &str) {
    if let Ok(entries) = fs::read_dir(from_path) {
        for entry in entries.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if let Some(Some(file_name)) = path.file_name().map(|name| name.to_str()) {
                if let Some(path_name) = path.to_str() {
                    let new_path = format!("{}/{}", to_path, file_name);

                    if path.is_dir() {
                        if fs::create_dir(&new_path).is_ok() || Path::new(&new_path).exists() {
                            copy_directory(path_name, &new_path);
                        } else {
                            println!("Failed to copy directory {} to {}", path_name, new_path);
                        }
                    } else {
                        if fs::copy(&path, Path::new(&new_path)).is_err() {
                            println!("Failed to copy {} to {}", path_name, new_path);
                        }
                    }
                }
            }
        }
    }
}

fn no_browser(matches: &clap::ArgMatches) -> bool {
    matches.occurrences_of("quiet") == 1 || !webbrowser::open("http://localhost:9000/").is_ok()
}
