use async_trait::async_trait;
use bytes::buf::BufExt as _;
use futures::future;
use futures::stream::{self, BoxStream, StreamExt};
use http::{Request, StatusCode};
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::platforms::shopify::requests::get_products::{GetProductsRequest, GetProductsResponse};

#[async_trait]
trait Client: Sync {
    async fn request<Req, Res>(&self, req: Req) -> Result<Res, Error>
    where
        Req: Into<Request<hyper::body::Body>> + Send,
        Res: DeserializeOwned;

    async fn get_products_page(
        &self,
        req: &GetProductsRequest,
    ) -> Result<GetProductsResponse, Error> {
        self.request(req).await
    }

    async fn get_products_stream<'a>(
        &'a self,
        req: &GetProductsRequest,
    ) -> BoxStream<'a, Result<GetProductsResponse, Error>> {
        Box::pin(
            stream::unfold(req.clone(), move |req| async move {
                req.next().map(|next| (req, next))
            })
            .then(move |req| async move { self.get_products_page(&req).await })
            .take_while(|res| {
                let ok = res
                    .as_ref()
                    .map(GetProductsResponse::non_empty)
                    .contains(&true);
                future::ready(ok)
            }),
        )
    }
}

#[cfg(test)]
mod client_tests {
    use super::*;
    use http::uri::Authority;
    use http::Uri;
    use std::collections::HashMap;

    const EMPTY_GET_PRODUCTS: &str = r#"{"products":[]}"#;
    const ONE_GET_PRODUCTS: &str = r#"{"products":[{"id":0,"title":"","handle":"","body_html":"","published_at":"","created_at":"","updated_at":"","vendor":"","product_type":"","tags":[],"variants":[],"images":[],"options":[]}]}"#;

    struct TestClient {
        request_to_json: HashMap<Uri, String>,
    }

    #[async_trait]
    impl Client for TestClient {
        async fn request<Req, Res>(&self, req: Req) -> Result<Res, Error>
        where
            Req: Into<Request<hyper::body::Body>> + Send,
            Res: DeserializeOwned,
        {
            let req = req.into();
            let json = self.request_to_json.get(req.uri()).ok_or(Error {
                msg: "404".to_string(),
            })?;
            serde_json::from_str(json).map_err(Error::from)
        }
    }

    #[tokio::test]
    async fn get_products_page_empty() {
        let req = GetProductsRequest::first_page(Authority::from_static("example.com"));
        let uri = Uri::from(&req);

        let mut request_to_json = HashMap::new();
        request_to_json.insert(uri, EMPTY_GET_PRODUCTS.to_string());
        let client = TestClient { request_to_json };

        let res = client.get_products_page(&req).await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), GetProductsResponse { products: vec![] });
    }

    #[tokio::test]
    async fn get_products_stream() {
        let req = GetProductsRequest::first_page(Authority::from_static("example.com"));

        let mut request_to_json = HashMap::new();
        request_to_json.insert(
            Uri::from_static("https://example.com/products.json?page=1&limit=250"),
            ONE_GET_PRODUCTS.to_string(),
        );
        request_to_json.insert(
            Uri::from_static("https://example.com/products.json?page=2&limit=250"),
            ONE_GET_PRODUCTS.to_string(),
        );
        request_to_json.insert(
            Uri::from_static("https://example.com/products.json?page=3&limit=250"),
            EMPTY_GET_PRODUCTS.to_string(),
        );
        let client = TestClient { request_to_json };

        let mut stream = client.get_products_stream(&req).await;

        assert!(stream.next().await.is_some());
        assert!(stream.next().await.is_some());
        assert!(stream.next().await.is_none());
    }
}

#[derive(Clone)]
struct HyperClient {
    client: hyper::client::Client<
        hyper_tls::HttpsConnector<hyper::client::HttpConnector>,
        hyper::body::Body,
    >,
}

impl Default for HyperClient {
    fn default() -> HyperClient {
        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::Client::builder().build(https);
        HyperClient { client }
    }
}

#[async_trait]
impl Client for HyperClient {
    async fn request<Req, Res>(&self, req: Req) -> Result<Res, Error>
    where
        Req: Into<Request<hyper::body::Body>> + Send,
        Res: DeserializeOwned,
    {
        let response = self.client.request(req.into()).await?;

        match response.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(response).await?;
                serde_json::from_reader(body.reader()).map_err(Error::from)
            }
            status => Err(Error::new(
                format!("HyperClient: wrong status {}", status).to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod hyper_client_tests {
    use super::*;
    use crate::chainable::Chainable;
    use crate::platforms::shopify::requests::page::Page;
    use futures::future::join_all;
    use http::uri::Authority;
    use rand::seq::SliceRandom as _;
    use std::str::FromStr;

    #[tokio::test]
    async fn get_products_page() {
        // https://www.shopify.com/plus/customers
        let all_ok = vec![
            "100percentpure.com",
            "animalsmatter.com",
            "beardandblade.com.au",
            "beastmodeonline.com",
            "blenderseyewear.com",
            "boieusa.com",
            "bollandbranch.com",
            "bombas.com",
            "cambridgesatchel.com",
            "campusprotein.com",
            "chubbiesshorts.com",
            "culturekings.com.au",
            "deathwishcoffee.com",
            "desmondanddempsey.com",
            "emazinglights.com",
            "emmabridgewater.com",
            "goodamerican.com",
            "graceandlace.com",
            "gymshark.com",
            "hawkersco.com",
            "hismileteeth.com",
            "honeybirdette.com",
            "ima-usa.com",
            "jungalow.com",
            "lairdsuperfood.com",
            "leesa.com",
            "lesportsac.com",
            "montedesign.com",
            "pand.co",
            "peepers.com",
            "petalandpup.com.au",
            "puravidabracelets.com",
            "rebeccaminkoff.com",
            "shop.lindt.ca",
            "shop.magnolia.com",
            "simbasleep.com",
            "sofa-club.co.uk",
            "the5th.co",
            "truevintage.com",
            "www.staples.ca",
        ]
        .choose_multiple(&mut rand::thread_rng(), 3)
        .map(|url| async move {
            let authority = Authority::from_str(url).unwrap();
            let page = Page::new(1, 3).unwrap();
            let req = GetProductsRequest::new(authority, page);

            HyperClient::default().get_products_page(&req).await
        })
        .pipe(join_all)
        .await
        .iter()
        .all(Result::is_ok);

        assert!(all_ok);
    }
}
