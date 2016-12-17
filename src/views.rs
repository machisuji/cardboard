extern crate iron;
extern crate hoedown;
extern crate hyper;
extern crate handlebars_iron;
extern crate rustc_serialize;

use self::iron::prelude::*;
use self::iron::status;

use self::hoedown::{Markdown, Render};
use self::hoedown::renderer::html::{Flags, Html};

use std::fs;
use std::io::prelude::*;
use std::fs::File;

use linked_hash_map::LinkedHashMap;

use yaml_utils;
use configuration;
use configuration::Config;

use self::handlebars_iron::Template;
use self::rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Board {
    id: String,
    label: String,
    cards: Vec<Card>
}

#[derive(Debug, Clone)]
pub struct Card {
    file_name: String,
    title: String,
    board: String,
    tags: Vec<String>,
    markdown: String,
    html: String
}

#[derive(Debug, Clone)]
pub struct Index {
    title: String,
    boards: Vec<Board>
}

impl ToJson for Index {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();

        m.insert("title".to_string(), self.title.to_json());
        m.insert("boards".to_string(), self.boards.to_json());

        m.to_json()
    }
}

impl ToJson for Board {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();

        m.insert("id".to_string(), self.id.to_json());
        m.insert("label".to_string(), self.label.to_json());
        m.insert("cards".to_string(), self.cards.to_json());

        m.to_json()
    }
}

impl ToJson for Card {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();

        m.insert("file_name".to_string(), self.file_name.to_json());
        m.insert("title".to_string(), self.title.to_json());
        m.insert("board".to_string(), self.board.to_json());
        m.insert("tags".to_string(), self.tags.to_json());
        m.insert("markdown".to_string(), self.markdown.to_json());
        m.insert("html".to_string(), self.html.to_json());

        m.to_json()
    }
}

impl Card {
    fn on_stack(&self) -> Card {
        let tags: Vec<String> = self.tags
            .iter()
            .map(|tag| tag.to_string())
            .collect();

        Card {
            file_name: self.file_name.to_string(),
            title: self.title.to_string(),
            board: self.board.to_string(),
            tags: tags,
            markdown: self.markdown.to_string(),
            html: self.html.to_string()
        }
    }
}

pub fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let config: Config = configuration::config();
    let cards: Vec<Card> = load_cards();

    let boards: Vec<Board> = config.boards.iter().map(|e| {
        let (id, label) = e;

        let board_cards: Vec<Card> = cards
            .iter()
            .filter(|card| &card.board == id)
            .map(|card| card.on_stack())
            .collect();

        Board {
            id: id.to_string(),
            label: label.to_string(),
            cards: board_cards
        }
    }).collect();

    let data = Index {
        title: String::from("Cardboard - Index"),
        boards: boards
    };

    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);

    Ok(resp)
}

fn load_cards() -> Vec<Card> {
    let mut cards = Vec::new();

    for card in fs::read_dir(".cardboard/cards").ok().unwrap().flat_map(|dir| dir.map(|e| e.path())) {
        let mut file = File::open(& card).ok().unwrap();
        let mut source = String::new();
        let mut markdown = String::new();
        let mut output = String::new();
        let meta: LinkedHashMap<String, String>;
        let mut card_title = String::from("");
        let file_name: String = card.file_name().unwrap().to_str().unwrap().to_string();

        if file.read_to_string(&mut source).is_ok() {
            let meta_yaml: String = source
                .lines()
                .take_while(|line| line.starts_with("meta:") || line.starts_with(" "))
                .collect::<Vec<&str>>()
                .join("\n");

            markdown = String::new();

            let markdown_body: String = source
                .lines()
                .skip_while( |line| {
                    let skip = configuration::is_meta_yaml(*line);

                    if skip == false { // first line if content: # Card Title
                        card_title = line[1..].to_string();
                    }

                    skip
                })
                .collect::<Vec<&str>>()
                .join("\n");

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
            markdown: markdown,
            html: output
        };

        cards.push(card);
    }

    cards
}
