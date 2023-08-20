// The `Sender` handles are moved into the tasks. As there are two
// tasks, we need a second `Sender`.
let tx2 = tx.clone();

// Spawn two tasks, one gets a key, the other sets a key
let t1 = tokio::spawn(async move {
    let cmd = Command::Get {
        key: "foo".to_string(),
    };

    tx.send(cmd).await.unwrap();
});

let t2 = tokio::spawn(async move {
    let cmd = Command::Set {
        key: "foo".to_string(),
        val: "bar".into(),
    };

    tx2.send(cmd).await.unwrap();
});