

#[tokio::main]
async fn main() -> Result<(),  Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    use std::sync::LazyLock;
    use bytes::Bytes;
    use http_body_util::{BodyExt, Full};
    use hyper::Request;
    use hyper_tls::HttpsConnector;
    use hyper_util::client::legacy::Client;
    use hyper_util::client::legacy::connect::HttpConnector;
    use hyper_util::rt::TokioExecutor;
    use serde_json::Value;

    static H: LazyLock<Client<HttpsConnector<HttpConnector>, Full<Bytes>>> = LazyLock::new(|| Client::builder(TokioExecutor::new()).build(HttpsConnector::new()));
    
    #[tokio::test]
    async fn foo() -> Result<(),  Box<dyn std::error::Error + Send + Sync>> {
        println!("hello world");
        Ok(())
    }

    #[tokio::test]
    async fn bar() -> Result<(),  Box<dyn std::error::Error + Send + Sync>> {
        let req = Request::builder()
            .uri("https://www.rust-lang.org/")
            .method("GET")
            .body(Full::default())?;
        let response = H.request(req).await?;
        println!("{:?} {:?}", response.version(), response.status());
        Ok(())
    }

    #[tokio::test]
    async fn boo() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let req = Request::builder()
            .uri("https://lhlzgroup.com/prod-api/cms/web/article/channel/news/view/5?siteId=1887062639954173953")
            .method("GET")
            .body(Full::default())?;
        let response = H.request(req).await?;
        let body = response.into_body().collect().await?.to_bytes();
        let value = serde_json::from_slice::<Value>(&body)?;
        println!("{:?}", value);
        Ok(())
    }
}