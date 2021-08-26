use tokio::io::{self, AsyncReadExt};

use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    let n = f.read(&mut buffer[..]).await?;
    println!("read result: {:?}", &buffer[..n]);

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    println!("read_to_end result: {:?}", buffer);

    Ok(())
}
