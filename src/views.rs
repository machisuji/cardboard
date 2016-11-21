extern crate iron;
extern crate hoedown;
extern crate hyper;

use self::iron::prelude::*;
use self::iron::status;

use self::hyper::header::{ContentType};
use self::hyper::mime::{Mime, TopLevel, SubLevel};

use self::hoedown::{Markdown, Render};
use self::hoedown::renderer::html::{Flags, Html};

use std::fs;
use std::io::prelude::*;
use std::fs::File;

pub fn index(_: &mut Request) -> IronResult<Response> {
    let mut html = String::with_capacity(4096);

    render_response(String::from("Cardboard - Index"), &mut html);

    html_response(html)
}

fn render_response(title: String, output: &mut String) {
    output.push_str("<html><head><title>");
    output.push_str(& title);
    output.push_str("</title>");

    output.push_str("<link rel=\"stylesheet\" href=\"http://yui.yahooapis.com/pure/0.6.0/pure-min.css\">");
    output.push_str("<link rel=\"stylesheet\" type=\"text/css\" href=\"main.css\">");
    output.push_str("</head><body>");

    render_menu(output);
    render_boards(output);

    output.push_str("</body>");
}

fn render_menu(output: &mut String) {
    output.push_str("
        <div class=\"pure-menu pure-menu-horizontal\">
            <a href=\"/\" class=\"pure-menu-heading pure-menu-link\">cardboard</a>
            <ul class=\"pure-menu-list\">
                <li class=\"pure-menu-item\"><a href=\"#\" class=\"pure-menu-link\">Boards</a></li>
            </ul>
        </div>
    ");
}

fn render_boards(output: &mut String) {
    output.push_str("<div class=\"pure-g\">");

    output.push_str("<div class=\"pure-u-1-3\">");
    render_cards(output);
    output.push_str("</div>");

    output.push_str("<div class=\"pure-u-1-3\">foobar</div>");
    output.push_str("<div class=\"pure-u-1-3\">foobar</div>");
    output.push_str("<div class=\"pure-u-1-3\">foobar</div>");
    output.push_str("<div class=\"pure-u-1-3\">foobar</div>");
    output.push_str("<div class=\"pure-u-1-3\">foobar</div>");

    output.push_str("</div>");
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
