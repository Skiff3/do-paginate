use std::cmp::{max, min};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pages {
    offset: usize,
    length: usize,
    limit: usize,
    f: fn(usize, usize) -> String,
}

impl Pages {
    pub fn new(length: usize, limit: usize, f: Option<fn(usize, usize) -> String>) -> Pages {
        Pages {
            offset: 0,
            length,
            limit,
            f: f.unwrap_or(|_: usize, _| -> String { "".to_string() }),
        }
    }

    pub fn with_offset(&self, offset: usize) -> Page {
        let mut page = Page::default();
        page.offset = offset;
        page.start = min(page.offset * self.limit, self.length);
        page.end = min(page.start + self.limit, self.length);
        page.length = max(page.end - page.start, 0);

        if page.length == 0 {
            page.start = 0;
            page.end = 0;
        };
        if page.length > 0 {
            page.end -= 1;
        };
        page.html = (self.f)(page.start, page.length);
        page
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn page_count(&self) -> usize {
        (self.length + self.limit - 1) / self.limit
    }
}

impl Iterator for Pages {
    type Item = Page;
    fn next(&mut self) -> Option<Self::Item> {
        let page: Page = self.with_offset(self.offset);
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
        self.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page {
    pub offset: usize,
    pub length: usize,
    pub start: usize,
    pub end: usize,
    html: String,
}

impl Page {
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            offset: 0usize,
            length: 0usize,
            start: 0usize,
            end: 0usize,
            html: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{Page, Pages};

    #[test]
    fn default_page() {
        let page = Page::default();
        assert_eq!(
            page,
            Page {
                offset: 0,
                length: 0,
                start: 0,
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
            pages.with_offset(0),
            Page {
                offset: 0,
                length: 5,
                start: 0,
                end: 4,
                html: "".to_string()
            }
        );
        assert_eq!(
            pages.with_offset(1),
            Page {
                offset: 1,
                length: 5,
                start: 5,
                end: 9,
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
                    "{}{}{}{}",
                    s,
                    "<a href=\"www.test.com/".to_string(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            pages.with_offset(0),
            Page {
                offset: 0,
                length: 0,
                start: 0,
                end: 0,
                html: "".to_string()
            }
        );
        assert_eq!(
            pages.with_offset(1),
            Page {
                offset: 1,
                length: 0,
                start: 0,
                end: 0,
                html: "".to_string()
            }
        );
    }

    #[test]
    fn limitless_page() {
        let total_items = 5usize;
        let items_per_page = 0usize;
        let f: fn(usize, usize) -> String = |x, y| -> String {
            (x..x + y).fold("".to_string(), |s: String, t: usize| {
                format!(
                    "{}{}{}{}",
                    s,
                    "<a href=\"www.test.com/".to_string(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            pages.with_offset(0),
            Page {
                offset: 0,
                length: 0,
                start: 0,
                end: 0,
                html: "".to_string()
            }
        );
        assert_eq!(
            pages.with_offset(1),
            Page {
                offset: 1,
                length: 0,
                start: 0,
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
                    "{}{}{}{}",
                    s,
                    "<a href=\"www.test.com/".to_string(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            pages.with_offset(0),
            Page {
                offset: 0,
                length: 5,
                start: 0,
                end: 4,
                html: "<a href=\"www.test.com/0\"></a></br><a href=\"www.test.com/1\"></a></br><a href=\"www.test.com/2\"></a></br><a href=\"www.test.com/3\"></a></br><a href=\"www.test.com/4\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            pages.with_offset(1),
            Page {
                offset: 1,
                length: 0,
                start: 0,
                end: 0,
                html: "".to_string()
            }
        );
        assert_eq!(
            pages.with_offset(2),
            Page {
                offset: 2,
                length: 0,
                start: 0,
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
                    "{}{}{}{}",
                    s,
                    "<a href=\"www.test.com/".to_string(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            pages.with_offset(0),
            Page {
                offset: 0,
                length: 1,
                start: 0,
                end: 0,
                html: "<a href=\"www.test.com/0\"></a></br>".to_string()
            }
        );
        assert_eq!(
            pages.with_offset(1),
            Page {
                offset: 1,
                length: 0,
                start: 0,
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
                    "{}{}{}{}",
                    s,
                    "<a href=\"www.test.com/".to_string(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            pages.with_offset(0),
            Page {
                offset: 0,
                length: 2,
                start: 0,
                end: 1,
                html: "<a href=\"www.test.com/0\"></a></br><a href=\"www.test.com/1\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            pages.with_offset(1),
            Page {
                offset: 1,
                length: 2,
                start: 2,
                end: 3,
                html: "<a href=\"www.test.com/2\"></a></br><a href=\"www.test.com/3\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            pages.with_offset(2),
            Page {
                offset: 2,
                length: 1,
                start: 4,
                end: 4,
                html: "<a href=\"www.test.com/4\"></a></br>".to_string()
            }
        );
        assert_eq!(
            pages.with_offset(3),
            Page {
                offset: 3,
                length: 0,
                start: 0,
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
                    "{}{}{}{}",
                    s,
                    "<a href=\"www.test.com/".to_string(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));

        assert_eq!(
            pages.with_offset(0),
            Page {
                offset: 0,
                length: 2,
                start: 0,
                end: 1,
                html: "<a href=\"www.test.com/0\"></a></br><a href=\"www.test.com/1\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            pages.with_offset(1),
            Page {
                offset: 1,
                length: 2,
                start: 2,
                end: 3,
                html: "<a href=\"www.test.com/2\"></a></br><a href=\"www.test.com/3\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            pages.with_offset(2),
            Page {
                offset: 2,
                length: 2,
                start: 4,
                end: 5,
                html: "<a href=\"www.test.com/4\"></a></br><a href=\"www.test.com/5\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            pages.with_offset(3),
            Page {
                offset: 3,
                length: 0,
                start: 0,
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
                    "{}{}{}{}",
                    s,
                    "<a href=\"www.test.com/".to_string(),
                    t.to_string(),
                    "\"></a></br>".to_string()
                )
            })
        };
        let pages = Pages::new(total_items, items_per_page, Some(f));
        assert_eq!(
            pages.with_offset(0),
            Page {
                offset: 0,
                length: 3,
                start: 0,
                end: 2,
                html: "<a href=\"www.test.com/0\"></a></br><a href=\"www.test.com/1\"></a></br><a href=\"www.test.com/2\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            pages.with_offset(1),
            Page {
                offset: 1,
                length: 2,
                start: 3,
                end: 4,
                html: "<a href=\"www.test.com/3\"></a></br><a href=\"www.test.com/4\"></a></br>"
                    .to_string()
            }
        );
        assert_eq!(
            pages.with_offset(2),
            Page {
                offset: 2,
                length: 0,
                start: 0,
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
                    "{}{}{}{}",
                    s,
                    "<a href=\"www.test.com/".to_string(),
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
                    start: 0,
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
                    "{}{}{}{}",
                    s,
                    "<a href=\"www.test.com/".to_string(),
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
                    start: 0,
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
        assert_eq!(5, pages.limit());
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
