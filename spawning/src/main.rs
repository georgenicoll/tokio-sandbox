use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;


#[tokio::main]
async fn main() {
    //Bind listener to the address
    let listener = TcpListener::bind("0.0.0.0:6379").await.unwrap();

    some_func().await;

    loop {
        //the second item contains ip and port of the new connection
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // A hashmap is used to store data
    let mut db = HashMap::new();

    //The connection lets us read/write redis **frames** instead of byte streams
    let mut connection = Connection::new(socket);

    //read frame to to receive command
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    //expects bytes... covered later in tutorial
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            },
            cmd => panic!("unimplemented: {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}

async fn some_func() {
    let v = vec![1, 2, 3];

    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });
}

