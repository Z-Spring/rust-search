#![allow(unused)]
#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
mod utils; // notice this!

#[get("/")]
fn hello() -> &'static str {
    "hello world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("you typed in: {}", cmd);
    let command = utils::get_command_from_query_string(&cmd);
    let redirect_url = match command {
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };
    Redirect::to(redirect_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, search])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_form_query_string_no_whitespace() {
        let actual = utils::get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_command_form_query_string_with_whitespace() {
        let actual = utils::get_command_from_query_string("tw @justOnce");
        let expected = "tw";
        assert_eq!(expected, actual);
    }
}
