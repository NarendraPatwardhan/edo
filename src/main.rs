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
    let val = manager.fetch_results().await;
    let mut successes = 0;
    let mut failures = 0;
    for v in val {
        match v {
            Ok(_) => successes += 1,
            Err(_) => failures += 1,
        }
    }
    println!(
        "Process completed with {} successes and {} failures",
        successes, failures
    );
    Ok(())
}
