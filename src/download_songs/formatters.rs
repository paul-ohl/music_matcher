pub fn ygg_torrent_search_formatter(query: &str) -> String {
    format!(
        "https://www.yggtorrent.top/engine/search?name={}{}{}",
        query.replace(" ", "+"),
        "&description=&file=&uploader=&category=2139&sub_category=2148",
        "&do=search&order=desc&sort=seed"
    )
}

pub fn bandcamp_search_formatter(query: &str) -> String {
    format!("https://bandcamp.com/search?q={}", query.replace(" ", "+"))
}
