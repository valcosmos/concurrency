use std::{sync::mpsc, thread, time::Duration};

use anyhow::{anyhow, Result};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建 NUM_PRODUCERS 个生产者线程
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    // 创建一个消费者线程 consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer received {:?}", msg);
        }
        println!("consumer exit");
        "xxxx_secret"
    });

    consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        // println!("producer {} sent {}", idx, value);
        thread::sleep(Duration::from_millis(sleep_time));

        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
