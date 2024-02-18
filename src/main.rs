fn main() {
    // target the website
    let response = reqwest::blocking::get("https://rust-trends.com/newsletter/rust-in-action-10-project-ideas-to-elevate-your-skills/");

    // extract content from the above response
    let html_content = response.unwrap().text().unwrap();
    print!("{html_content}");
}