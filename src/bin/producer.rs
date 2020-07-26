use amiquip::{Connection, Exchange, Publish, Result};
use std::{thread, time};

fn main() -> Result<()> {
    // Wait for Rabbit to be up
    println!("Waiting 30 seconds for Rabbitmq");
    thread::sleep(time::Duration::from_secs(30));
    println!("Done waiting for Rabbitmq");

    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@rabbitmq:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new("hello there".as_bytes(), "hello"))?;

    connection.close()
}