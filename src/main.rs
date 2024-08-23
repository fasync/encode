use clap::{Arg, Command};
use std::str;
use base64::{engine::general_purpose, Engine as _};

fn hex_encode(value: &str) -> String {
    hex::encode(value)
}

fn base64_encode(value: &str) -> String {
    general_purpose::STANDARD.encode(value)
}

fn base64_decode(value: &str) -> String {
    let decoded_bytes = general_purpose::STANDARD.decode(value).unwrap();
    str::from_utf8(&decoded_bytes).unwrap().to_string()
}

fn html_encode(value: &str) -> String {
    html_escape::encode_text(value).to_string()
}

fn url_encode(value: &str) -> String {
    urlencoding::encode(value).to_string()
}

fn parse(matches: &clap::ArgMatches, value: &str, func: fn(&str) -> String) {
    if let Some(val) = matches.get_one::<String>(value) {
        match val.parse::<String>() {
            Ok(s) => println!("{}", func(&s)),
            Err(_) => println!("Could not parse requested string for {} encoding.", value),
        }
    }
}

fn main() {
    let matches = Command::new("encode")
        .version("0.0.1")
        .author("Florian Büstgens <fbuestgens@mailbox.org>")
        .about("Encode string to different encodings")
        .arg(
            Arg::new("url")
                .long("url")
                .help("URL encoding"),
        )
        .arg(
            Arg::new("html")
                .long("html")
                .help("HTML encoding"),
        )
        .arg(
            Arg::new("hex")
                .long("hex")
                .help("Hex encoding"),
        )
        .arg(
            Arg::new("ebase64")
                .long("ebase64")
                .help("Base64 encoding"),
        )
        .arg(
            Arg::new("dbase64")
                .long("dbase64")
                .help("Base64 decoding"),
        )
        .get_matches();

    parse(&matches, "url", url_encode);
    parse(&matches, "html", html_encode);
    parse(&matches, "hex", hex_encode);
    parse(&matches, "ebase64", base64_encode);
    parse(&matches, "dbase64", base64_decode);
}
