tokio::spawn(async move {
    let (mut rd, mut wr) = socket.split();

    if io::copy(&mut rd, &mut wr).await.is_err() {
        eprintln!("failed to copy");
    }
});