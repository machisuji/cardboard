extern crate iron;
extern crate hyper;
extern crate params;

use self::iron::prelude::*;
use self::iron::status;

use self::hyper::header::{ContentType};
use self::hyper::mime::{Mime, TopLevel, SubLevel};
use self::hyper::status::StatusCode;

use self::params::{Params, Value};

use git;
use text_files;

pub fn update_card(request: &mut Request) -> IronResult<Response> {
    if let Ok(params) = request.get_ref::<Params>() {
        if let Some(&Value::String(ref board)) = params.find(&["card", "board"]) {
            if let Some(&Value::String(ref file_name)) = params.find(&["card", "file_name"]) {
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

                if text_files::update_text_file(& format!(".cardboard/cards/{}", &file_name), update).is_ok() {
                    let repo = git::open(".cardboard");
                    let sha = git::commit_file(
                        &format!("cards/{}", &file_name),
                        &format!("Moved {} to {}", &file_name, &board),
                        &repo
                    );

                    if sha.is_ok() {
                        json_response(r##"{"message": "Card updated"}"##.to_string())
                    } else {
                        json_response_with_status(
                            format!(r##"{{"message": "commit failed: {}"}}"##, sha.err().unwrap()),
                            status::InternalServerError
                        )
                    }
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
