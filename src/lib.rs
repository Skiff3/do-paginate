use std::cmp::{max, min};

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct OutOfBound;

impl fmt::Display for OutOfBound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "out of bound")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pages {
    offset: usize,
    length: usize,
    per_page: usize,
    html_function: fn(usize, usize) -> String,
}

impl Pages {
    pub fn new(length: usize, per_page: usize, f: Option<fn(usize, usize) -> String>) -> Pages {
        Pages {
            offset: 0,
            length,
            per_page,
            html_function: f.unwrap_or(|_, _| -> String { "".to_string() }),
        }
    }

    pub fn to_page_number(&self, offset: usize) -> Result<Page, OutOfBound> {
        let mut page = Page::default();

        if offset > self.page_count() {
            return Err(OutOfBound);
        }
        page.offset = offset;
        page.begin = min(page.offset * self.per_page, self.length);
        page.end = min(page.begin + self.per_page, self.length);
        page.length = max(page.end - page.begin, 0);

        if page.length == 0 {
            page.begin = 0;
            page.end = 0;
        };
        if page.length > 0 {
            page.end -= 1;
        };
        page.html = (self.html_function)(page.begin, page.length);
        Ok(page)
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn per_page(&self) -> usize {
        self.per_page
    }

    pub fn page_count(&self) -> usize {
        (self.length + self.per_page - 1) / self.per_page
    }
}

impl Iterator for Pages {
    type Item = Page;
    fn next(&mut self) -> Option<Self::Item> {
        let page: Page = match self.to_page_number(self.offset) {
            Ok(page) => page,
            Err(msg) => {
                panic!("{:#?}", msg)
            }
        };
        self.offset += 1;
        if page.is_empty() {
            None
        } else {
            Some(page)
        }
    }
}

impl IntoIterator for &Pages {
    type Item = Page;
    type IntoIter = Pages;

    fn into_iter(self) -> Pages {
        Pages {
            offset: 0,
            length: self.length(),
            per_page: self.per_page(),
            html_function: self.html_function,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Page {
    pub offset: usize,
    pub length: usize,
    pub begin: usize,
    pub end: usize,
    html: String,
}

impl Page {
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[cfg(test)]
mod tests {

    use super::{Page, Pages};
    use std::fmt::{self, Error};

    fn get_url() -> String {
        "www.test.com/".to_string()
    }

    #[test]
    fn test_iter() {
        let pages = Pages::new(10, 2, None);
        let mut list_page: Vec<Page> = Vec::new();
        let iter = pages.into_iter();
        for page in iter {
            // eprintln!("{:#?}", page);
            list_page.push(page);
        }
        assert_eq!(list_page.len(), 5);
    }

    #[test]
    fn default_page() {
        let page = Page::default();
        assert_eq!(
            page,
            Page {
                offset: 0,
                length: 0,
                begin: 0,
                end: 0,
                html: "".to_string()
            }
        );
    }

    #[test]
    fn no_function_passing() {
        let total_items = 10usize;
        let items_per_page = 5usize;

        let pages = Pages::new(total_items, items_per_page, None);
        assert_eq!(
            match pages.to_page_number(0) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 0,
                length: 5,
                begin: 0,
                end: 4,
                html: "".to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(1) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 1,
                length: 5,
                begin: 5,
                end: 9,
                html: "".to_string()
            }
        );
    }

    #[test]
    fn out_of_bound() {
        let total_items = 0usize;
        let items_per_page = 5usize;
        let pages = Pages::new(total_items, items_per_page, None);
        let page = match pages.to_page_number(1) {
            Ok(page) => page,
            Err(msg) => {
                eprint!("{}", msg);
                Page::default()
            }
        };
        assert_eq!(
            page,
            Page {
                offset: 0,
                length: 0,
                begin: 0,
                end: 0,
                html: "".to_string()
            }
        );
    }

    #[test]
    fn empty_page() {
        let total_items = 0usize;
        let items_per_page = 5usize;

        let f: fn(usize, usize) -> String = |x, y| -> String {
            (x..x + y).fold("".to_string(), |s: String, t: usize| {
                format!(
                    "{}{}{}{}{}",
                    s,
                    "<a href=\"",
                    get_url(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            match pages.to_page_number(0) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 0,
                length: 0,
                begin: 0,
                end: 0,
                html: "".to_string()
            }
        );
    }

    #[test]
    fn single_page() {
        let total_items = 5usize;
        let items_per_page = 5usize;
        let f: fn(usize, usize) -> String = |x, y| -> String {
            (x..x + y).fold("".to_string(), |s: String, t: usize| {
                format!(
                    "{}{}{}{}{}",
                    s,
                    "<a href=\"",
                    get_url(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            match pages.to_page_number(0) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 0,
                length: 5,
                begin: 0,
                end: 4,
                html: "<a href=\"www.test.com/0\"></a></br><a href=\"www.test.com/1\"></a></br><a href=\"www.test.com/2\"></a></br><a href=\"www.test.com/3\"></a></br><a href=\"www.test.com/4\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(1) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 1,
                length: 0,
                begin: 0,
                end: 0,
                html: "".to_string()
            }
        );
    }

    #[test]
    fn single_item() {
        let total_items = 1usize;
        let items_per_page = 5usize;

        let f: fn(usize, usize) -> String = |x, y| -> String {
            (x..x + y).fold("".to_string(), |s: String, t: usize| {
                format!(
                    "{}{}{}{}{}",
                    s,
                    "<a href=\"",
                    get_url(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            match pages.to_page_number(0) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 0,
                length: 1,
                begin: 0,
                end: 0,
                html: "<a href=\"www.test.com/0\"></a></br>".to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(1) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 1,
                length: 0,
                begin: 0,
                end: 0,
                html: "".to_string()
            }
        );
    }

    #[test]
    fn odd_items() {
        let total_items = 5usize;
        let items_per_page = 2usize;
        let f: fn(usize, usize) -> String = |x, y| -> String {
            (x..x + y).fold("".to_string(), |s: String, t: usize| {
                format!(
                    "{}{}{}{}{}",
                    s,
                    "<a href=\"",
                    get_url(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            match pages.to_page_number(0) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 0,
                length: 2,
                begin: 0,
                end: 1,
                html: "<a href=\"www.test.com/0\"></a></br><a href=\"www.test.com/1\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(1) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 1,
                length: 2,
                begin: 2,
                end: 3,
                html: "<a href=\"www.test.com/2\"></a></br><a href=\"www.test.com/3\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(2) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 2,
                length: 1,
                begin: 4,
                end: 4,
                html: "<a href=\"www.test.com/4\"></a></br>".to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(3) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 3,
                length: 0,
                begin: 0,
                end: 0,
                html: "".to_string()
            }
        );
    }

    #[test]
    fn even_items() {
        let total_items = 6usize;
        let items_per_page = 2usize;

        let f: fn(usize, usize) -> String = |x, y| -> String {
            (x..x + y).fold("".to_string(), |s: String, t: usize| {
                format!(
                    "{}{}{}{}{}",
                    s,
                    "<a href=\"",
                    get_url(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));

        assert_eq!(
            match pages.to_page_number(0) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 0,
                length: 2,
                begin: 0,
                end: 1,
                html: "<a href=\"www.test.com/0\"></a></br><a href=\"www.test.com/1\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(1) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 1,
                length: 2,
                begin: 2,
                end: 3,
                html: "<a href=\"www.test.com/2\"></a></br><a href=\"www.test.com/3\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(2) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 2,
                length: 2,
                begin: 4,
                end: 5,
                html: "<a href=\"www.test.com/4\"></a></br><a href=\"www.test.com/5\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(3) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 3,
                length: 0,
                begin: 0,
                end: 0,
                html: "".to_string()
            }
        );
    }

    #[test]
    fn odd_sizes() {
        let total_items = 5usize;
        let items_per_page = 3usize;
        let f: fn(usize, usize) -> String = |x, y| -> String {
            (x..x + y).fold("".to_string(), |s: String, t: usize| {
                format!(
                    "{}{}{}{}{}",
                    s,
                    "<a href=\"",
                    get_url(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            match pages.to_page_number(0) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 0,
                length: 3,
                begin: 0,
                end: 2,
                html: "<a href=\"www.test.com/0\"></a></br><a href=\"www.test.com/1\"></a></br><a href=\"www.test.com/2\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(1) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 1,
                length: 2,
                begin: 3,
                end: 4,
                html: "<a href=\"www.test.com/3\"></a></br><a href=\"www.test.com/4\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            match pages.to_page_number(2) {
                Ok(page) => page,
                Err(msg) => {
                    eprint!("{}", msg);
                    Page::default()
                }
            },
            Page {
                offset: 2,
                length: 0,
                begin: 0,
                end: 0,
                html: "".to_string()
            }
        );
    }

    #[test]
    fn iterator() {
        let total_items = 1usize;
        let items_per_page = 1usize;
        let f: fn(usize, usize) -> String = |x, y| -> String {
            (x..x + y).fold("".to_string(), |s: String, t: usize| {
                format!(
                    "{}{}{}{}{}",
                    s,
                    "<a href=\"",
                    get_url(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        for p in pages {
            assert_eq!(
                p,
                Page {
                    offset: 0,
                    length: 1,
                    begin: 0,
                    end: 0,
                    html: "<a href=\"www.test.com/0\"></a></br>".to_string()
                }
            );
        }
    }

    #[test]
    fn iterator_ref() {
        let total_items = 1usize;
        let items_per_page = 1usize;
        let f: fn(usize, usize) -> String = |x, y| -> String {
            (x..x + y).fold("".to_string(), |s: String, t: usize| {
                format!(
                    "{}{}{}{}{}",
                    s,
                    "<a href=\"",
                    get_url(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        for p in &pages {
            assert_eq!(
                p,
                Page {
                    offset: 0,
                    length: 1,
                    begin: 0,
                    end: 0,
                    html: "<a href=\"www.test.com/0\"></a></br>".to_string()
                }
            );
        }
    }

    #[test]
    fn is_empty() {
        let empty_page = Page::default();
        assert!(empty_page.is_empty());

        let filled_page = Page {
            length: 1,
            ..Page::default()
        };
        assert!(!filled_page.is_empty());
    }

    #[test]
    fn offset() {
        let pages = Pages::new(100, 5, None);
        assert_eq!(0, pages.offset());
    }

    #[test]
    fn length() {
        let pages = Pages::new(100, 5, None);
        assert_eq!(100, pages.length());
    }

    #[test]
    fn limit() {
        let pages = Pages::new(100, 5, None);
        assert_eq!(5, pages.per_page());
    }

    #[test]
    fn page_count() {
        let pages = Pages::new(100, 5, None);
        assert_eq!(20, pages.page_count());

        let pages = Pages::new(101, 5, None);
        assert_eq!(21, pages.page_count());

        let pages = Pages::new(99, 5, None);
        assert_eq!(20, pages.page_count());
    }
}