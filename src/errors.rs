use std::io;
use std::collections::HashSet;
use tendril::StrTendril;
use hyper::error::Error as HyperError;

quick_error! {
    #[derive(Debug)]
    pub enum LinkCheckerError {
        Io(err: io::Error) {
            from()
            cause(err)
            description(err.description())
            display("Couldn't read file")
        }
        Read {
            from()
            description("Error reading file to tendril string.")
        }
        MissingLinks(links: HashSet<StrTendril>) {
            description("Missing links")
            display("Missing links: [{}]",
                links.iter()
                    .map(|key| format!("\"{}\"", key))
                    .collect::<Vec<_>>().join(", ")
            )
        }
        Http(url: String, err: HyperError) {
            from(err)
            cause(err)
            description("Error fetching external link")
            display("Error fetching external link `{}`: {}", url, err)
        }
    }
}
