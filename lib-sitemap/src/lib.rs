extern crate async_std;
extern crate futures;
extern crate log;
extern crate reqwest;
extern crate select;
extern crate serde_json;

use futures::future;

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

    async fn parse(&mut self, links: Vec<String>) {
        future::join_all(links.iter()
            .filter(|link| !self.table.contains_key(link.as_str()))
            .map(|borrow_link| async {
                let link = Box::new(borrow_link.clone());
                let response = match reqwest::get((*link).as_str()).await {
                     Result::Ok(resp) => resp,
                     Result::Err(e) => {
                         error!("{}", e);
                         return Option::None
                    }
                };
                let body = match response.text().await {
                     Result::Ok(body) => body,
                     Result::Err(e) => {
                         error!("{}", e);
                         return Option::None
                    }
                };

                let children: Vec<_> = Document::from(body.as_str())
                    .find(Name("a"))
                    .filter_map(|n| n.attr("href"))
                    .map(String::from)
                    .collect();

                self.table.insert(String::from(*link), children.clone());
                self.parse(children);
                Option::Some(())
            }).collect::<Vec<_>>());
    }

    async fn start(&mut self) {
        self.parse(vec![self.root_link.clone()]).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_drop() {
        assert_eq!(2 + 2, 4);
    }
}
