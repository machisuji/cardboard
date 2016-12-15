extern crate iron;
extern crate hyper;
extern crate params;

use self::iron::prelude::*;
use self::iron::status;

use self::hyper::header::{ContentType};
use self::hyper::mime::{Mime, TopLevel, SubLevel};
use self::hyper::status::StatusCode;

use self::params::{Params, Value};

use text_files;
use configuration;

pub fn update_card(request: &mut Request) -> IronResult<Response> {
    if let Ok(params) = request.get_ref::<Params>() {
        if let Some(&Value::String(ref board)) = params.find(&["card", "board"]) {
            let config = configuration::config();

            let update = |input: String, output: &mut String| {
                for line in input.lines() {
                    if line.starts_with("  board:") {
                        output.push_str("  board: ");
                        output.push_str(board);
                    } else {
                        output.push_str(line);
                    }

                    output.push_str("\n");
                }
            };

            if text_files::update_text_file(".cardboard/cards/boards.md", update).is_ok() {
                json_response(r##"{"message": "Card updated"}"##.to_string())
            } else {
                json_response_with_status(
                    r##"{"message": "Failed to update card"}"##.to_string(),
                    status::BadRequest)
            }
        } else {
            json_response_with_status(
                format!(r##"{{"message": "Cannot update: {:?}"}}"##, params).to_string(),
                status::BadRequest)
        }
    } else {
        json_response_with_status(
            r##"{"message": "Expected JSON request body"}"##.to_string(),
            status::BadRequest)
    }
}

fn json_response(content: String) -> IronResult<Response> {
    json_response_with_status(content, status::Ok)
}

fn json_response_with_status(content: String, status: StatusCode) -> IronResult<Response> {
    let mut response = Response::with((status, content));

    response.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Json, vec![])));

    Ok(response)
}
