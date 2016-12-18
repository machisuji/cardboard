extern crate iron;
extern crate hyper;
extern crate params;
extern crate regex;

use self::iron::prelude::*;
use self::iron::status;

use self::hyper::header::{ContentType};
use self::hyper::mime::{Mime, TopLevel, SubLevel};
use self::hyper::status::StatusCode;

use self::params::{Params, Value};

use self::regex::Regex;

use git;
use text_files;
use configuration;

pub fn update_card(request: &mut Request) -> IronResult<Response> {
    if let Ok(params) = request.get_ref::<Params>() {
        if let Some(&Value::String(ref file_name)) = params.find(&["card", "file_name"]) {
            if let Some(&Value::String(ref board)) = params.find(&["card", "board"]) {
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
            } else if let Some(&Value::String(ref content)) = params.find(&["card", "content"]) {
                let update = |input: String, output: &mut String| {
                    let meta = configuration::read_meta_yaml(input);

                    output.push_str(&meta);
                    output.push_str("\n\n");
                    output.push_str(content);
                    output.push_str("\n");
                };

                if text_files::update_text_file(& format!(".cardboard/cards/{}", &file_name), update).is_ok() {
                    let repo = git::open(".cardboard");
                    let sha = git::commit_file(
                        &format!("cards/{}", &file_name),
                        &format!("Updated content of {}", &file_name),
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

pub fn create_card(request: &mut Request) -> IronResult<Response> {
    if let Ok(params) = request.get_ref::<Params>() {
        if let Some(&Value::String(ref board)) = params.find(&["card", "board"]) {
            if let Some(&Value::String(ref content)) = params.find(&["card", "content"]) {
                let mut output: String = String::new();

                output.push_str("meta:\n");
                output.push_str("  board: ");
                output.push_str(board);
                output.push_str("\n");
                output.push_str("  tags: feature\n"); // tags are not actually used yet
                output.push_str("\n");
                output.push_str(content);
                output.push_str("\n");

                let regex: Regex = Regex::new(r"[^\w]+").unwrap();
                let trim: Regex = Regex::new(r"(^_)|(_$)").unwrap();
                let header: String = content.lines().next().unwrap_or("# New card").to_string();
                let title: String = header[1..].trim().to_lowercase();
                let name: String = trim.replace_all(&regex.replace_all(&title, "_"), "");
                let file_name: &str = &format!("{}.md", name);

                if text_files::create_text_file(& format!(".cardboard/cards/{}", &file_name), output).is_ok() {
                    let repo = git::open(".cardboard");
                    let sha = git::commit_file(
                        &format!("cards/{}", &file_name),
                        &format!("Created {} in {}", &file_name, &board),
                        &repo
                    );

                    if sha.is_ok() {
                        json_response(r##"{"message": "Card created"}"##.to_string())
                    } else {
                        json_response_with_status(
                            format!(r##"{{"message": "commit failed: {}"}}"##, sha.err().unwrap()),
                            status::InternalServerError
                        )
                    }
                } else {
                    json_response_with_status(
                        r##"{"message": "Failed to create card"}"##.to_string(),
                        status::BadRequest)
                }
            } else {
                json_response_with_status(
                    format!(r##"{{"message": "Cannot create card (missing content)"}}"##).to_string(),
                    status::BadRequest)
            }
        } else {
            json_response_with_status(
                format!(r##"{{"message": "Cannot create card (missing board)"}}"##).to_string(),
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
