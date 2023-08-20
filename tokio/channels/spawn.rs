use mini_redis::client;
// The `move` keyword is used to **move** ownership of `rx` into the task.
let manager = tokio::spawn(async move {
    // Establish a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // Start receiving messages
    while let Some(cmd) = rx.recv().await {
        use Command::*;

        match cmd {
            Get { key } => {
                client.get(&key).await;
            }
            Set { key, val } => {
                client.set(&key, val).await;
            }
        }
    }
});