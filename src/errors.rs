use std::io;
use std::collections::HashSet;
use tendril::StrTendril;

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
    }
}
