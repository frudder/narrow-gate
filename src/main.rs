
#[tokio::main]
async fn main() -> Result<(),  Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");
    Ok(())
}


#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn foo() -> Result<(),  Box<dyn std::error::Error + Send + Sync>> {
        println!("hello world");
        Ok(())
    }
}