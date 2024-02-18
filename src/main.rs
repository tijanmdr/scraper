struct BlogContent {
    title: String, 
    time: String, 
    content: String
}

fn main() {
    // target the website
    let response = reqwest::blocking::get("https://rust-trends.com/posts/database-crates-diesel-sqlx-tokio-postgress/");

    // extract content from the above response
    let html_content = response.unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&html_content);

    let title_selector = scraper::Selector::parse("div.page-header").unwrap();
    let title = document.select(&title_selector)
        .next()
        .map(|div| div.text().collect::<String>())
        .unwrap();

    let time_selector = scraper::Selector::parse("time").unwrap();
    let time = document.select(&time_selector)
        .next()
        .map(|time| time.text().collect::<String>())
        .unwrap();

    let content_selector = scraper::Selector::parse("section.body").unwrap();
    let content = document.select(&content_selector)
        .next()
        .map(|section| section.text().collect::<String>())
        .unwrap();
    
    let _blog = BlogContent {
        title, 
        time, 
        content
    };

    println!("{}", _blog.title);


}