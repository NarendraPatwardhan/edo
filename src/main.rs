mod origin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = origin::TokioManager::new(10);
    for i in 0..10 {
        let tx = manager.spawn_transmitter();
        tokio::spawn(async move {
            if i % 2 == 0 {
                tx.send(Ok(i)).await.unwrap();
            } else {
                tx.send(Err(i)).await.unwrap();
            }
        });
    }
    manager.close_transmission();
    let val = manager.aggregate_results().await;
    println!("{:?}", val);
    Ok(())
}
