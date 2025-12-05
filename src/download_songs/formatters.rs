pub fn ygg_torrent_search_formatter(query: &str) -> String {
    format!(
        "https://www.yggtorrent.top/engine/search?name={}&description=&file=&uploader=&category=2139&sub_category=2148&do=search",
        query.replace(" ", "+")
    )
}

pub fn bandcamp_search_formatter(query: &str) -> String {
    format!("https://bandcamp.com/search?q={}", query.replace(" ", "+"))
}
