use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use::rand::Rng;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let rx = Arc::new(Mutex::new(rx));

    for workers_id in 1..=3 {
        let rx_clone = Arc::clone(&rx);
        thread::spawn(move || {
            loop {
                let task = {
                    let lock = rx_clone.lock().unwrap();
                    lock.recv()
                };

                match task {
                    Ok(task) => {
                        println!("Worker {} got a task: {}", workers_id, task);

                        let delay = rand::thread_rng().gen_range(100..=500);
                        thread::sleep(std::time::Duration::from_millis(delay));
                    }

                    Err(err) => {
                        println!("Worker {} got a error: {:?}", workers_id, err);
                        break;
                    }
                }
            }

            println!("Worker {} is done", workers_id);
        });
    }

    for i in 1..=10 {
        let task = format!("Task {}", i);
        println!("Main thread sending: {}", task);
        tx.send(task).unwrap();
    }

    drop(tx);

    thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread is done");
}


