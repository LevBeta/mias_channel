use mias_channel::responder::ResponderMessage;
use tokio::sync::mpsc::Receiver;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mias_channel::responder_channel::<i64, f64>(10);

    tokio::task::spawn(async move {
        Converter::start(&mut rx).await;
    });

    let res = tx.send(10).await;

    println!("{:?}", res);
}

struct Converter;

impl Converter {
    async fn start(rx: &mut Receiver<ResponderMessage<i64, f64>>) {
        while let Some((req, tx)) = rx.recv().await {
            tx.send(Self::convert(req)).unwrap();
        }
    }

    fn convert(req: i64) -> f64 {
        req as f64
    }
}
