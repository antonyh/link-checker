/* link-checker tool - https://github.com/antonyh/link-checker
   Build using Rust 1.3
   Produced by Antony Hutchison - https://github.com/antonyh
   See also, http://antonyh.co.uk/
 */

extern crate hyper;

use std::env;
use std::io::prelude::*;
use std::io;
use std::io::BufReader;
use std::fs::File;

use hyper::Client;

fn main() {
    // Check for parameter, if none show help and exit.
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Usage: link-checker <textfile>");
            return;
        }
    };

    let success = process_file(input_file);
    match success {
        Ok(_) => { println!("\nSuccess."); },
        Err(e) => { println!("\nError encountered: {}", e); }
    }
}

// Make a request to a URL and print the status.
// Anything other than OK is a fail for the report.
fn check_url(url: String) {
    let client = Client::new();
    let res = client.get(&url[..]).send();
    let status = res.unwrap().status;
    match status {
        hyper::Ok => {
            println!("{} => OK", url);
        },
        _ => {
            println!("{} => FAIL with status: {}" , url, status);
        }
    }
}

// Process a file.
// Prepend http:// if it's missing.
// Call check_url for each URL line in the file.
fn process_file(input_file :String) -> Result<(), io::Error> {
    let f = try!(File::open(input_file));
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let mut url = l.to_string();
        if "http" != &l[0..4] {
            url = format!("http://{}", l)
        }
        check_url(url);
    }
    Ok(())
}
