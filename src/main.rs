use scraper::{Html, Selector};
use std::{fs::File, io::Read};
use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("What airline will you check?\n1:Korean Air\n2:Delta Air\n3:Air Canada");
    input! {
        number: i32
    }

    let file_name = if number == 1 {
        "korean.html"
    } else if number == 2 {
        "delta.html"
    } else {
        "canada.html"
    };

    let mut  file = File::open(file_name)?;
    let mut contens = String::new();
    file.read_to_string(&mut contens)?;
    let html = &contens;

    let element = css_select(&html).unwrap();
    
    println!("{}",element);
    Ok(())
}

fn css_select(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse("div.wrapper > main.main > div.layout > div#flightSearch > div > div.page-body > div.section > section").unwrap();
    let element = fragment.select(&selector).next().unwrap().inner_html();
    Ok(element)
}