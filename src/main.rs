use clap::{Arg, App, ArgMatches};
use std::str;

fn hex_encode(value: &str) -> String {
    hex::encode(value)
}

fn base64_encode(value: &str) -> String {
    base64::encode(value)
}

fn base64_decode(value: &str) -> String {
    str::from_utf8(&base64::decode(value).unwrap()).unwrap().to_string()
}

fn html_encode(value: &str) -> String {
    html_escape::encode_text(value).to_string()
}

fn url_encode(value: &str) -> String {
    urlencoding::encode(value).to_string()
}

fn parse(app: &ArgMatches, value: &str, func: fn(&str) -> String) {
    match app.value_of(value) {
	None => {},
	Some(value) => {
	    match value.parse::<String>() {
		Ok(s) => println!("{}", func(&s)),
		Err(_) => println!("Could not parse requested string for {} encoding.", value)
	    }
	}
    }
}

fn main() {
    let args = App::new("encode")
        .version("0.0.1")
        .author("Florian Büstgens <fbuestgens@mailbox.org>")
        .about("Encode string to different encodings")
        .arg(Arg::with_name("url").short("u").long("url").takes_value(true).help("url encoding"))
        .arg(Arg::with_name("html").short("h").long("html").takes_value(true).help("html encoding"))
        .arg(Arg::with_name("hex").short("x").long("hex").takes_value(true).help("hex encoding"))
        .arg(Arg::with_name("ebase64").short("eb64").long("ebase64").takes_value(true).help("base64 encoding"))
        .arg(Arg::with_name("dbase64").short("db64").long("dbase64").takes_value(true).help("base64 decoding"))
        .get_matches();

    parse(&args, "url", url_encode);
    parse(&args, "html", html_encode);
    parse(&args, "hex", hex_encode);
    parse(&args, "ebase64", base64_encode);
    parse(&args, "dbase64", base64_decode);
}
