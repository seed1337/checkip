// print options, 1, 2 | 1 is exit | 2 is show ip
// when user chooses option 2, make a request to ip site, convert the data then print ip

use std::io;
use std::io::Read;
use error_chain::error_chain;
use serde_json::Value;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() {
    let menu: &str = "1: save yourself\n2: dox yourself";
    println!("{}", menu);
    println!("wat u wanna do");
    let mut choice = String::new();

    while choice.trim() != "1" {
        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => std::process::exit(1),
            "2" => {
                println!("sending ip to doxbin...");
                let _ = get_ip();
                println!("here it is, have fun getting doxxed");
                std::process::exit(1);
            },
            _ => println!("cmon bro chill out and use the program"),
        }
    }
}

#[tokio::main]
async fn get_ip() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    let json: Value = res.json().await?;
    if let Some(origin) = json.get("origin").and_then(|value| value.as_str()) {
        println!("im stealing ur ip: {}", origin);
    } else {
        println!("where tf ur ip at?");
    }

    Ok(())
}
