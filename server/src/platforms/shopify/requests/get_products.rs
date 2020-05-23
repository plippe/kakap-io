use http::method::Method;
use http::uri::{Authority, Builder, PathAndQuery, Scheme, Uri};
use http::Request;
use hyper::Body;

use crate::chainable::Chainable;
use crate::platforms::shopify::requests::page::Page;
use crate::platforms::shopify::schema::Product;

#[derive(Clone, Debug)]
pub struct GetProductsRequest {
    pub authority: Authority,
    pub page: Page,
}

impl GetProductsRequest {
    pub fn new(authority: Authority, page: Page) -> Self {
        GetProductsRequest { authority, page }
    }

    pub fn first_page(authority: Authority) -> Self {
        Self::new(authority, Page::first())
    }

    pub fn next(&self) -> Option<Self> {
        self.page.next().map(|page| GetProductsRequest {
            page,
            authority: self.authority.clone(),
        })
    }
}

impl From<&GetProductsRequest> for Uri {
    fn from(it: &GetProductsRequest) -> Uri {
        let path_and_query = format!(
            "/products.json?page={}&limit={}",
            it.page.page_number, it.page.items_per_page
        )
        .pipe(PathAndQuery::from_maybe_shared)
        .unwrap();

        Builder::new()
            .scheme(Scheme::HTTPS)
            .authority(it.authority.clone())
            .path_and_query(path_and_query)
            .build()
            .unwrap()
    }
}

impl From<&GetProductsRequest> for Method {
    fn from(_: &GetProductsRequest) -> Method {
        Method::GET
    }
}

impl From<&GetProductsRequest> for Request<Body> {
    fn from(it: &GetProductsRequest) -> Request<Body> {
        Request::builder()
            .method(Method::from(it))
            .uri(Uri::from(it))
            .header("user-agent", "Kakap.io/0.1")
            .body(Body::empty())
            .unwrap()
    }
}

#[cfg(test)]
mod get_products_request_tests {
    use super::*;

    #[test]
    fn first_page() {
        let res = GetProductsRequest::first_page(Authority::from_static("example.com"));
        assert_eq!(res.page, Page::first());
    }

    #[test]
    fn next() {
        let a = GetProductsRequest::first_page(Authority::from_static("example.com"));
        let b = a.clone().next().unwrap();

        assert_eq!(a.authority, b.authority);
        assert_eq!(a.page.clone().next().unwrap(), b.page);
    }

    #[test]
    fn into_uri() {
        let req = GetProductsRequest::first_page(Authority::from_static("example.com"));
        let uri = Uri::from(&req);

        assert_eq!(uri.scheme_str(), Some("https"));
        assert_eq!(uri.host(), Some("example.com"));
        assert_eq!(uri.path(), "/products.json");
        assert_eq!(uri.query(), Some("page=1&limit=250"));
    }

    #[test]
    fn into_method() {
        let req = GetProductsRequest::first_page(Authority::from_static("example.com"));
        let method = Method::from(&req);

        assert_eq!(method, Method::GET)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GetProductsResponse {
    pub products: Vec<Product>,
}

impl GetProductsResponse {
    pub fn is_empty(&self) -> bool {
        self.products.is_empty()
    }

    pub fn non_empty(&self) -> bool {
        !self.is_empty()
    }
}

#[cfg(test)]
mod get_products_response_tests {
    use super::*;

    #[test]
    fn is_empty() {
        let is_empty = GetProductsResponse { products: vec![] };
        let non_empty = GetProductsResponse {
            products: vec![Product {
                id: 0,
                title: "".to_string(),
                handle: "".to_string(),
                body_html: None,
                published_at: "".to_string(),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                vendor: "".to_string(),
                product_type: "".to_string(),
                tags: vec![],
                variants: vec![],
                images: vec![],
                options: vec![],
            }],
        };

        assert!(is_empty.is_empty());
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn non_empty() {
        let is_empty = GetProductsResponse { products: vec![] };
        let non_empty = GetProductsResponse {
            products: vec![Product {
                id: 0,
                title: "".to_string(),
                handle: "".to_string(),
                body_html: None,
                published_at: "".to_string(),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                vendor: "".to_string(),
                product_type: "".to_string(),
                tags: vec![],
                variants: vec![],
                images: vec![],
                options: vec![],
            }],
        };

        assert!(!is_empty.non_empty());
        assert!(non_empty.non_empty());
    }
}
