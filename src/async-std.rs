use async_std::sync::channel;
use async_std::task::{sleep, spawn};
use std::time::Duration;

#[derive(Debug)]
enum Message {
    SendWelcomeEmail { to: String },
    DownloadVideo { id: usize },
    GenerateReport,
    Terminate,
}

#[async_std::main]
async fn main() {
    let (sender, receiver) = channel(5);

    let size = 5;
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
        let receiver = receiver.clone();
        let worker = spawn(async move {
            loop {
                let message = receiver.recv().await.unwrap_or_else(|_| Message::Terminate);
                println!("Worker {}: {:?}", id, message);
                match message {
                    Message::Terminate => break,
                    _ => sleep(Duration::from_secs(1 + id as u64)).await,
                }
            }
        });
        workers.push(worker);
    }

    sender.send(Message::DownloadVideo { id: 10 }).await;
    sender.send(Message::GenerateReport).await;
    sender
        .send(Message::SendWelcomeEmail {
            to: "hi@example.com".into(),
        })
        .await;
    sender.send(Message::DownloadVideo { id: 92 }).await;

    for _ in &workers {
        let _ = sender.send(Message::Terminate).await;
    }
    for worker in workers {
        let _ = worker.await;
    }
}
