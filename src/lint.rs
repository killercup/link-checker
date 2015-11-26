use std::collections::HashSet;
use regex::Regex;

pub fn lint_html_links(html: &str) -> Result<(), HashSet<String>> {
    let link_regex = Regex::new("href=(\"|')#(?P<id>.*?)(\"|')").unwrap();
    let id_regex = Regex::new("id=(\"|')(?P<id>.*?)(\"|')").unwrap();

    let links: HashSet<&str> = link_regex.captures_iter(html)
                                         .filter_map(|cap| cap.name("id"))
                                         .collect();

    let ids: HashSet<&str> = id_regex.captures_iter(html)
                                     .filter_map(|cap| cap.name("id"))
                                     .collect();

    if links.is_subset(&ids) {
        Ok(())
    } else {
        Err(links.difference(&ids)
                 .cloned()
                 .map(String::from)
                 .collect::<HashSet<_>>())
    }
}
