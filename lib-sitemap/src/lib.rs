extern crate log;
extern crate serde_json;

use log::{error, info};

use std::collections::HashMap;
use std::io;
use std::ops::Drop;

struct SitemapGenerator<T: io::Write + Copy> {
    table: HashMap<String, Vec<String>>,
    output_src: T,
    root_link: String,
}

impl<T: io::Write + Copy> Drop for SitemapGenerator<T> {
    fn drop(&mut self) {
        match serde_json::to_writer_pretty(self.output_src, &self.table) {
            Ok(_) => info!("Successfully written to input source"),
            Err(e) => error!("{}", e),
        }
    }
}

impl<T: io::Write + Copy> SitemapGenerator<T> {
    fn new(link: &str, output_src: T) -> SitemapGenerator<T> {
        SitemapGenerator {
            table: HashMap::new(),
            output_src: output_src,
            root_link: String::from(link),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_drop() {
        assert_eq!(2 + 2, 4);
    }
}
