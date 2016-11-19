extern crate iron;
extern crate hoedown;
extern crate hyper;
extern crate webbrowser;
extern crate router;
extern crate staticfile;

use iron::prelude::*;
use iron::status;

use hoedown::{Markdown, Render};
use hoedown::renderer::html::{Flags, Html};

use hyper::header::{ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::fs;

use router::Router;
use staticfile::Static;

fn main() {
    fn make_router() -> Router {
        let mut router = Router::new();

        router.get("/", index, "index");
        router.get("*", file_handler(), "files");

        router
    }

    fn index(_: &mut Request) -> IronResult<Response> {
        let mut html = String::with_capacity(4096);

        render_response(String::from("Cardboard - Index"), &mut html);

        html_response(html)
    }

    fn file_handler() -> Static {
        Static::new(Path::new("public/"))
    }

    fn render_response(title: String, output: &mut String) {
        output.push_str("<html><head><title>");
        output.push_str(& title);
        output.push_str("</title>");

        output.push_str("<link rel=\"stylesheet\" type=\"text/css\" href=\"main.css\">");
        output.push_str("</head><body>");

        render_cards(output);

        output.push_str("</body>");
    }

    fn render_cards(output: &mut String) {
        for card in fs::read_dir(".cardboard/cards").ok().unwrap().flat_map(|dir| dir.map(|e| e.path())) {
            let mut file = File::open(& card).ok().unwrap();
            let mut source = String::new();

            output.push_str("<div class=\"card\">");

            if file.read_to_string(&mut source).is_ok() {
                let doc = Markdown::new(&source);
                let mut html = Html::new(Flags::empty(), 0);
                let result = html.render(&doc);

                output.push_str(result.to_str().unwrap());
            } else {
                output.push_str("Could not read: ");
                output.push_str(card.to_str().unwrap());
            }

            output.push_str("</div>");
        }
    }

    fn html_response(content: String) -> IronResult<Response> {
        let mut response = Response::with((status::Ok, content));

        response.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));

        Ok(response)
    }

    fn check_data_directory() -> bool {
        let root = ".cardboard";
        let cards = root.to_string() + "/cards";

        (fs::create_dir(root).is_ok() && fs::create_dir(& cards).is_ok()) || Path::new(& cards).exists()
    }

    if check_data_directory() {
        println!("Listening on port 9000 ...");

        if !webbrowser::open("http://localhost:9000/").is_ok() {
            println!("Open http://localhost:9000 to view your project")
        }

        Iron::new(make_router()).http("localhost:9000").unwrap();
    } else {
        println!("Failed to create data directory");
    }
}
