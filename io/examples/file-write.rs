use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    let n = file.write(b"hello,world!!!").await?;
    println!("Wrote the first {} bytes.", n);

    let mut buffer = File::create("foo.txt").await?;
    buffer
        .write_all(b"hello,world!!!\nhello,world!!!\nhello,world!!!")
        .await?;
    println!("Wrote all");

    Ok(())
}
