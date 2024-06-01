use dotenv::dotenv;
use std::env;
use std::io::{self, BufRead};

fn main() {
    let mut terminal = true;
    while terminal {
        let mut line = String::new();
        let stdin = io::stdin();
        println!("{}", "1. authorize");
        println!("{}", "2. exit");
        stdin.lock().read_line(&mut line).unwrap();
        let option = line.trim();
        let one = "1";
        let two = "2";
        if option == one {
            authorize()
        } else if option == two {
            terminal = false;
        }
    }
}

fn authorize() {
    dotenv().ok();
    let authorize_url = "https://api.upstox.com/v2/login/authorization/dialog/";
    let client_id_string = "?client_id=";
    let client_id = env::var("CLIENT_ID").unwrap().to_string();
    let redirect_uri_string = "&redirect_uri=";
    let redirect_url = env::var("REDIRECT_URL").unwrap().to_string();
    let url = [
        authorize_url,
        client_id_string,
        &client_id,
        redirect_uri_string,
        &redirect_url,
    ]
    .join("");
    println!("{}", url);
    let _ = open::that(url);
}
