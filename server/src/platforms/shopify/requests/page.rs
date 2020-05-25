use crate::error::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Page {
    pub page_number: u8,
    pub items_per_page: u8,
}

impl Page {
    const MIN_PAGE_NUMBER: u8 = 1;
    const MAX_PAGE_NUMBER: u8 = u8::MAX;
    const MIN_ITEMS_PER_PAGE: u8 = 1;
    const MAX_ITEMS_PER_PAGE: u8 = 250;

    pub fn new(page_number: u8, items_per_page: u8) -> Result<Page, Error> {
        match (page_number, items_per_page) {
            (page_number, _) if page_number < Page::MIN_PAGE_NUMBER => Err(Error::new(format!(
                "Invalid page, must be {} or above",
                Page::MIN_PAGE_NUMBER
            ))),
            (_, items_per_page) if items_per_page < Page::MIN_ITEMS_PER_PAGE => {
                Err(Error::new(format!(
                    "Invalid limit, must be {} or above",
                    Page::MIN_ITEMS_PER_PAGE
                )))
            }
            (_, items_per_page) if items_per_page > Page::MAX_ITEMS_PER_PAGE => {
                Err(Error::new(format!(
                    "Invalid limit, must be {} or bellow",
                    Page::MAX_ITEMS_PER_PAGE
                )))
            }
            (page_number, items_per_page) => Ok(Page {
                page_number,
                items_per_page,
            }),
        }
    }
}

impl Page {
    pub fn first() -> Self {
        Page {
            page_number: Page::MIN_PAGE_NUMBER,
            items_per_page: Page::MAX_ITEMS_PER_PAGE,
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self.page_number == Self::MAX_PAGE_NUMBER {
            true => None,
            false => Some(Page {
                page_number: self.page_number + 1,
                ..*self
            }),
        }
    }
}

#[cfg(test)]
mod page_tests {
    use super::*;

    #[test]
    fn err_page_number_lower_than_min() {
        let res = Page::new(Page::MIN_PAGE_NUMBER - 1, Page::MIN_ITEMS_PER_PAGE);
        assert!(res.is_err());
    }

    #[test]
    fn err_items_per_page_lower_than_min() {
        let res = Page::new(Page::MIN_PAGE_NUMBER, Page::MIN_ITEMS_PER_PAGE - 1);
        assert!(res.is_err());
    }

    #[test]
    fn err_items_per_page_higher_than_max() {
        let res = Page::new(Page::MIN_PAGE_NUMBER, Page::MAX_ITEMS_PER_PAGE + 1);
        assert!(res.is_err());
    }

    #[test]
    fn first() {
        let res = Page::first();

        assert_eq!(res.page_number, Page::MIN_PAGE_NUMBER);
        assert_eq!(res.items_per_page, Page::MAX_ITEMS_PER_PAGE);
    }

    #[test]
    fn next() {
        let a = Page::first();
        let b = a.next();

        assert!(b.is_some());
        assert_eq!(a.page_number + 1, b.unwrap().page_number);
        assert_eq!(a.items_per_page, b.unwrap().items_per_page);
    }

    #[test]
    fn last_next() {
        let a = Page::new(Page::MAX_PAGE_NUMBER, Page::MAX_ITEMS_PER_PAGE).ok();
        let b = a.and_then(|a| a.next());

        assert!(a.is_some());
        assert!(b.is_none());
    }
}
