use dotenv::dotenv;
use serde_json::json;
use std::env;
use std::error::Error;
use std::io::{self, BufRead};

fn main() {
    let mut terminal = true;
    while terminal {
        let mut line = String::new();
        let stdin = io::stdin();
        println!("{}", "1. authorize");
        println!("{}", "2. login");
        println!("{}", "3. exit");
        stdin.lock().read_line(&mut line).unwrap();
        let option = line.trim();
        let one = "1";
        let two = "2";
        let three = "3";
        if option == one {
            authorize();
        } else if option == two {
            login();
        } else if option == three {
            terminal = false;
        }
    }
}

fn login() {
    async fn call() -> Result<(), Box<dyn Error>> {
        dotenv().ok();
        let code = env::var("CODE").unwrap().to_string();
        let client_id = env::var("CLIENT_ID").unwrap().to_string();
        let client_secret = env::var("API_SECRET").unwrap().to_string();
        let redirect_url = env::var("REDIRECT_URL").unwrap().to_string();

        let data = json!({
            "code": code,
            "client_id": client_id,
            "client_secret": client_secret,
            "redirect_uri":redirect_url,
            "grant_type": "authorization_code",
        });
        let resp = reqwest::Client::new()
            .post("https://api.upstox.com/v2/login/authorization/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("accept", "application/json")
            .query(&data)
            .send()
            .await?;
        let resp_text = resp.text().await?;
        let user_info: serde_json::Value = serde_json::from_str(&resp_text).unwrap();
        let access_token = user_info["access_token"].clone();
        println!("{}", access_token);
        Ok(())
    }
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(call());
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
