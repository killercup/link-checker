use std::io;
use std::collections::HashSet;

quick_error! {
    #[derive(Debug)]
    pub enum LinkCheckerError {
        Io(err: io::Error) {
            from()
            cause(err)
            description(err.description())
            display("Couldn't read file")
        }
        MissingLinks(links: HashSet<String>) {
            description("Missing links")
            display("Missing links: {:#?}", links)
        }
    }
}
