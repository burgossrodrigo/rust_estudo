use ::tokio;
use rand::Rng;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);

    let worker = tokio::spawn(async move {
        while let Some(task) = rx.recv().await {
            println!("Worker got a task: {}", task);

            let delay = rand::thread_rng().gen_range(100..=200);

            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
        }

        println!("Worker finished");
    });

    // let tx_clone = tx.clone();

    let num_producers = 3;
    let mut producer_handles = vec![];

    for producer_id in 0..num_producers {
        let tx_clone = tx.clone();
        let producer_handler = tokio::spawn(async move {
            for i in 0..10 {
                let task = format!("Task {} from producer {}", i, producer_id);
                println!("Task {} from producer {} sent", i, producer_id);
                tx_clone.send(task).await.unwrap();
            }

            println!("Producer {} finished", producer_id);
        });

        producer_handles.push(producer_handler);
    }

    for handler in producer_handles {
        handler.await.unwrap();
    }

    drop(tx);

    worker.await.unwrap();
}
