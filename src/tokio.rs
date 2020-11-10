use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Debug)]
enum Message {
    SendWelcomeEmail { to: String },
    DownloadVideo { id: usize },
    GenerateReport,
    Terminate,
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = unbounded_channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let size = 5;
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
        let receiver = Arc::clone(&receiver);
        let worker = tokio::spawn(async move {
            loop {
                let message = receiver
                    .lock()
                    .await
                    .recv()
                    .await
                    .unwrap_or_else(|| Message::Terminate);
                println!("Worker {}: {:?}", id, message);
                match message {
                    Message::Terminate => break,
                    _ => sleep(Duration::from_secs(1 + id as u64)).await,
                }
            }
        });
        workers.push(worker);
    }

    sender.send(Message::DownloadVideo { id: 10 }).unwrap();
    sender.send(Message::GenerateReport).unwrap();
    sender
        .send(Message::SendWelcomeEmail {
            to: "hi@example.com".into(),
        })
        .unwrap();
    sender.send(Message::DownloadVideo { id: 92 }).unwrap();

    for _ in &workers {
        let _ = sender.send(Message::Terminate);
    }
    for worker in workers {
        let _ = worker.await;
    }
}
