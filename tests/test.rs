#[cfg(test)]
mod tests {
    use axum::http;
    use axum::http::Uri;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


    #[tokio::test]
    async fn book() -> Result<()> {
        Ok(())
    }

    #[tokio::test]
    async fn log() -> Result<()> {
        tracing_subscriber::fmt::init();
        tracing::info!("test");
        Ok(())
    }

    #[tokio::test]
    async fn buf() -> Result<()> {
        let uri = "https://www.rust-lang.org/index.html".parse::<Uri>().unwrap();
        Ok(())
    }
}
