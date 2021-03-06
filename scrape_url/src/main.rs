use std::fs;
fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Fetching url: {}",url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output,md.as_bytes()).unwrap();
    println!("Convertd markdown has been saved in {}.",output);
}

// 错误处理

fn main2() -> Result<(), Box<dyn std::error::Error>> {

    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Fetching url: {}",url);

    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output,md.as_bytes())?;
    println!("Convertd markdown has been saved in {}.",output);
    Ok(())

}
