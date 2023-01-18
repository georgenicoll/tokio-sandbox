use bytes::Bytes;
use mini_redis::Result;
use mini_redis::client::{self, Client};

#[tokio::main]
async fn main() -> Result<()> {
    //Open a connection to mini-redis
    let mut client = client::connect("localhost:6379").await?;

    //Initial get...
    let value = get_and_output_value(&mut client, "hello").await?;
    let new_value: String = match value {
        Some(value) => String::from_utf8(value.to_vec()).unwrap() + "+",
        None => String::from("world"),
    };

    //set key "hello" to value "world"
    client.set("hello", new_value.into()).await?;

    //get the value for the "hello" key
    get_and_output_value(&mut client, "hello").await?;

    Ok(())
}

async fn get_and_output_value(client: &mut Client, key: &str) -> Result<Option<Bytes>> {
    let value = client.get(key).await?;
    println!("Got value from server: {} -> {:?}", key, value);
    Ok(value)
}
