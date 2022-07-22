//! Module `ggr` contains functionality for performing Google searches.

use std::io;

/// Does the ggr, ks.
///
/// (Translation: Opens a Google search link for `query` in the user's default browser.)
pub fn ggrks(query: &str) -> io::Result<()> {
    let url = get_google_search_url(query);
    webbrowser::open(&url)
}

const GOOGLE_SEARCH_URL_PREFIX: &str = "https://www.google.com/search?q=";

/// Given a search query, returns a Google Search URL for that query.
fn get_google_search_url(query: &str) -> String {
    // Allocate enough space for the prefix and query
    let mut url = String::with_capacity(GOOGLE_SEARCH_URL_PREFIX.len() + query.len());

    // Build the query URL
    url.push_str(GOOGLE_SEARCH_URL_PREFIX);
    // Testing seems to imply that Google simply doesn't care about special characters? 🤔
    url.push_str(query);

    url
}
