extern crate async_std;
extern crate futures;
extern crate log;
extern crate reqwest;
extern crate select;
extern crate serde_json;

use log::{error, info};

use std::collections::HashMap;
use std::io::Write;
use std::ops::Drop;

use select::document::Document;
use select::predicate::Name;

struct SitemapGenerator<T: Write + Copy> {
    table: HashMap<String, Vec<String>>,
    sink: T,
    root_link: String,
}

impl<T: Write + Copy> Drop for SitemapGenerator<T> {
    fn drop(&mut self) {
        match serde_json::to_writer_pretty(self.sink, &self.table) {
            Ok(_) => info!("Successfully written to input source"),
            Err(e) => error!("{}", e),
        }
    }
}

impl<T: Write + Copy> SitemapGenerator<T> {
    fn new(link: &str, sink: T) -> SitemapGenerator<T> {
        SitemapGenerator {
            table: HashMap::new(),
            sink: sink,
            root_link: String::from(link),
        }
    }

    fn parse(&mut self, links: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        for link in links.iter() {
            if !self.table.contains_key(link) {
                let body: &str = &reqwest::blocking::get(link)?.text()?;

                let children: Vec<_> = Document::from(body)
                    .find(Name("a"))
                    .filter_map(|n| n.attr("href"))
                    .map(String::from)
                    .collect();

                self.table.insert(String::from(link), children.clone());
                self.parse(&children)?;
            }
        }

        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.parse(&vec![self.root_link.clone()])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_drop() {
        assert_eq!(2 + 2, 4);
    }
}
