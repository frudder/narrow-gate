// mod net;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     env_logger::init();
//     println!("Hello, world!");
//     Ok(())
// }

// #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
// struct Book<'a> {
//     name: &'a str,
// }
//
// impl<'a> Book<'a> {
//     fn new(name: &'a str) -> Self {
//         Self { name }
//     }
// }
//
// impl std::fmt::Display for Book<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.name)
//     }
// }
//
// impl<'a> From<&'a str> for Book<'a> {
//     fn from(name: &'a str) -> Self {
//         Self { name }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use bytes::Bytes;
//     use http_body_util::{BodyExt, Full};
//     use hyper::Request;
//     use hyper_tls::HttpsConnector;
//     use hyper_util::client::legacy::connect::HttpConnector;
//     use hyper_util::client::legacy::Client;
//     use hyper_util::rt::{TokioExecutor, TokioTimer};
//     use serde_json::Value;
//     use std::arch::asm;
//     use std::path::Path;
//     use std::sync::LazyLock;
//     use std::time::Duration;
//     use log::info;
//
//
//     static H: LazyLock<Client<HttpsConnector<HttpConnector>, Full<Bytes>>> = LazyLock::new(|| {
//         Client::builder(TokioExecutor::new())
//             .pool_idle_timeout(Duration::from_secs(30))
//             .timer(TokioTimer::default())
//             .build(HttpsConnector::new())
//     });
//
//     #[tokio::test]
//     async fn foo() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         println!("hello world");
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn bar() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         let req = Request::builder()
//             .uri("https://www.rust-lang.org/")
//             .method("GET")
//             .body(Full::default())?;
//         let response = H.request(req).await?;
//         println!("{:?} {:?}", response.version(), response.status());
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn boo() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         let req = Request::builder()
//             .uri("https://lhlzgroup.com/prod-api/cms/web/article/channel/news/view/5?siteId=1887062639954173953")
//             .method("GET")
//             .body(Full::default())?;
//         let response = H.request(req).await?;
//         let body = response.into_body().collect().await?.to_bytes();
//         let value = serde_json::from_slice::<Value>(&body)?;
//         println!("{:?}", value);
//         Ok(())
//     }
//
//     #[test]
//     fn book() {
//         let n = Book::new("he");
//         let b = n.to_string();
//         println!("{}", b)
//         // println!("{:?}", n);
//         // let d = Book::default();
//         // println!("{:?}", d);
//     }
//
//     #[test]
//     fn into_book() {
//         let b: Book = "hello".into();
//         println!("{}", b);
//     }
//
//     #[test]
//     fn far() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         let fd = Path::new("20180223024228763.pdf");
//         let text = pdf_extract::extract_text(fd)?;
//         println!("{:?}", text);
//         Ok(())
//     }
//
//     #[test]
//     fn assembly() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         unsafe { asm!("nop") }
//         Ok(())
//     }
//
//     #[test]
//     fn encode() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         // let text = "Hello, World!";
//         // let v: String =  text.into();
//         // println!("{}", v);
//         // println!("{}", text);
//         Ok(())
//     }
//
//     #[test]
//     fn log() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         env_logger::init();
//         info!(logger: env_logger,  "test");
//         Ok(())
//     }
// }

// impl std::fmt::Display for Book<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.name)
//     }
// }

pub(crate) struct Book<'a>(&'a str);

fn main() {}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use std::any::Any;
    use std::iter::once;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[derive(Serialize, Deserialize)]
    struct Book<'a>(&'a str);

    impl std::fmt::Display for Book<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<'a> From<&'a str> for Book<'a> {
        fn from(name: &'a str) -> Self {
            Self(name)
        }
    }

    impl<'a> Book<'a> {
        fn new(name: &'a str) -> Self {
            Self(name)
        }
    }

    #[tokio::test]
    async fn foo() -> Result<()> {
        let b: Book = Book::from("v");
        // serde_json::json!()
        let c = serde_json::to_string(&b).unwrap();
        println!("{}", c);
        // let d = &b as &Box<dyn std::fmt::Display>;
        // let d = &b as &dyn std::fmt::Display;
        println!("{:}", b);
        Ok(())
    }

    #[tokio::test]
    async fn cmd() -> Result<()> {
        let output = std::process::Command::new("ping")
            .arg("www.baidu.com")
            .output()?;
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Ok(())
    }

    type Object = Box<dyn Any>;

    #[tokio::test]
    async fn df() -> Result<()> {
        // let fd = File::open("20250120.json").await?;
        // let df = LazyJsonLineReader::new(PlPath::new("20250120.json")).finish()?.collect()?;
        // println!("{:?}", df);
        // let fd = File::open("20250120.json")?;
        // let df = JsonReader::new(fd)
        //     .with_json_format(JsonFormat::Json)
        //     .finish()?;
        //
        // let f = df.column("author")?.eq(df.column("人间甜盐")?);
        //
        //
        // println!("{:?}", df);

        let n = &String::from("move");
        let book = move || {
            println!("fn {:?}", n);
            n
        };
        book();
        println!("orp {:?}", n);
        Ok(())
    }

    #[tokio::test]
    async fn bar() -> Result<()> {
        let v = once(1).into_iter().collect::<Vec<_>>();
        v.iter().for_each(|x| println!("{}", x));
        Ok(())
    }
}
