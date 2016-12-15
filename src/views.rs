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

use linked_hash_map::LinkedHashMap;

use yaml_utils;
use configuration;
use configuration::Config;

#[derive(Debug, Clone)]
pub struct Card {
    file_name: String,
    title: String,
    board: String,
    tags: Vec<String>,
    html: String
}

pub fn index(_: &mut Request) -> IronResult<Response> {
    let mut html = String::with_capacity(4096);

    render_response(String::from("Cardboard - Index"), &mut html);

    html_response(html)
}

fn render_response(title: String, output: &mut String) {
    output.push_str("<html><head><title>");
    output.push_str(& title);
    output.push_str("</title>");

    output.push_str("<link rel=\"stylesheet\" type=\"text/css\" href=\"pure-0.6.0.min.css\">");
    output.push_str("<link rel=\"stylesheet\" type=\"text/css\" href=\"main.css\">");
    output.push_str("<script src=\"jquery-3.1.1.min.js\"></script>");
    output.push_str("<script src=\"main.js\"></script>");
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

    render_cards(output);

    output.push_str("</div>");
}

fn render_cards(output: &mut String) {
    let config: Config = configuration::config();
    let cards: Vec<Card> = load_cards();

    for (board, label) in config.boards {
        output.push_str(& format!(
            "<div id=\"{}\" class=\"board pure-u-1-3\" ondrop=\"Cardboard.drop(event)\"
                  ondragover=\"Cardboard.allowDrop(event)\">",
            board
        ));
        output.push_str("<h2 class=\"board-name\">");
        output.push_str(label.as_str());
        output.push_str("</h2>");

        for i in 0..(cards.len() - 1) {
            let ref card = cards[i];

            if card.board == board {
                output.push_str(card.html.as_str());
            }
        }

        output.push_str("</div>");
    }
}

fn load_cards() -> Vec<Card> {
    let mut cards = Vec::new();

    for card in fs::read_dir(".cardboard/cards").ok().unwrap().flat_map(|dir| dir.map(|e| e.path())) {
        let mut file = File::open(& card).ok().unwrap();
        let mut source = String::new();
        let mut output = String::new();
        let meta: LinkedHashMap<String, String>;
        let mut card_title = String::from("");
        let file_name: String = card.file_name().unwrap().to_str().unwrap().to_string();

        if file.read_to_string(&mut source).is_ok() {
            let meta_yaml: String = source
                .lines()
                .skip_while(|line| !line.starts_with("meta:"))
                .take_while(|line| line.starts_with("meta:") || line.starts_with(" "))
                .collect::<Vec<&str>>()
                .join("\n");

            let mut markdown: String = source.lines().take(1).collect::<Vec<&str>>().join("\n");
            let markdown_body: String = source
                .lines()
                .skip_while( |line|
                    *line == markdown ||         // skip markdown title
                    line.starts_with("meta:") || // skip meta yaml
                    line.starts_with(" ") ||     // more meta yaml
                    *line == ""                  // possible empty lines
                )
                .collect::<Vec<&str>>()
                .join("\n");

            card_title = markdown[1..].to_string();

            output.push_str("<div id=\"");
            output.push_str(&file_name);
            output.push_str("\" class=\"card\" draggable=\"true\" ondragstart=\"Cardboard.drag(event)\">");

            markdown.push_str("\n\n");
            markdown.push_str(&markdown_body);

            let doc = Markdown::new(&markdown);
            let mut html = Html::new(Flags::empty(), 0);
            let result = html.render(&doc);

            output.push_str(result.to_str().unwrap());

            meta = yaml_utils::read_yaml_object(meta_yaml, "meta")
                .unwrap_or_else(|| LinkedHashMap::new());
        } else {
            meta = LinkedHashMap::new();

            output.push_str("Could not read: ");
            output.push_str(card.to_str().unwrap());
        }

        output.push_str("</div>");

        let card = Card {
            file_name: file_name,
            title: card_title,
            board: meta.get("board").unwrap_or(&String::from("unassigned")).to_string(),
            tags: meta
                .get("tags")
                .map(|tags|
                    tags
                    .split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|s| s.trim().to_owned())
                    .collect::<Vec<String>>()
                )
                .unwrap_or_else(|| vec![]),
            html: output
        };

        cards.push(card);
    }

    cards
}

fn html_response(content: String) -> IronResult<Response> {
    let mut response = Response::with((status::Ok, content));

    response.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));

    Ok(response)
}
