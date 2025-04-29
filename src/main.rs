

#[tokio::main]
async fn main() -> Result<(),  Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");
    Ok(())
}


#[cfg(test)]
mod tests {

    use http_body_util::Empty;
    use hyper::Request;
    use hyper_util::client::legacy::{connect::HttpConnector, Client};


    #[tokio::test]
    async fn foo() -> Result<(),  Box<dyn std::error::Error + Send + Sync>> {
        println!("hello world");
        Ok(())
    }

    #[tokio::test]
    async fn bar() -> Result<(),  Box<dyn std::error::Error + Send + Sync>> {
        
        let h = Client::builder(hyper_util::rt::TokioExecutor::new()).build(HttpConnector::new());
        
        let req = Request::builder()
            .uri("https://www.rust-lang.org/")
            .method("GET")
            .body(Empty::<bytes::Bytes>::new())?;


        Ok(())
    }
}