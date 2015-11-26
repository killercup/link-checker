use std::collections::HashSet;
use regex::Regex;

pub fn lint_html_links(html: &str) -> Result<(), HashSet<String>> {
    // let link_regex = Regex::new("href=(\"|')#(?P<id>.*?)(\"|')").unwrap();
    // let id_regex = Regex::new("id=(\"|')(?P<id>.*?)(\"|')").unwrap();

    let regex = Regex::new("href=(\"|')#(?P<anchor_link>.*?)(\"|')|id=(\"|')(?P<id>.*?)(\"|')")
                    .unwrap();

    let links: HashSet<&str> = regex.captures_iter(html)
                                    .filter_map(|cap| cap.name("anchor_link"))
                                    .collect();

    let ids: HashSet<&str> = regex.captures_iter(html)
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
