#![feature(async_await)]

type Fail = Box<dyn std::error::Error + Send + Sync + 'static>;

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Fail>{
    let text = surf::get("http://google.com").send_text().await?;
    dbg!(text);
    Ok(())
}
