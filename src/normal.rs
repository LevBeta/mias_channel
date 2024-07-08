use tokio::sync::mpsc;

use crate::error::{MiasChannelError, MiasChannelResult};

pub fn channel<R, S>(buffer: usize) -> (Sender<R, S>, Receiver<R, S>) {
    let (tx, rx) = mpsc::channel::<R>(buffer);
    let (response_tx, response_rx) = mpsc::channel::<S>(buffer);
    (Sender::new(tx, response_rx), Receiver::new(rx, response_tx))
}

pub struct Sender<R, S> {
    pub(crate) tx: mpsc::Sender<R>,
    pub(crate) rx: mpsc::Receiver<S>,
}

impl<R, S> Sender<R, S> {
    pub(crate) fn new(tx: mpsc::Sender<R>, rx: mpsc::Receiver<S>) -> Self {
        Self { tx, rx }
    }

    pub async fn send(&mut self, request: R) -> MiasChannelResult<()> {
        self.tx
            .send(request)
            .await
            .map_err(|e| MiasChannelError::SendError(e.to_string()))
    }

    pub async fn recv(&mut self) -> MiasChannelResult<S> {
        self.rx.recv().await.ok_or(MiasChannelError::RecvError(
            "Failed to receive message".to_string(),
        ))
    }
}

pub struct Receiver<R, S> {
    pub(crate) rx: mpsc::Receiver<R>,
    pub(crate) tx: mpsc::Sender<S>,
}

impl<R, S> Receiver<R, S> {
    pub(crate) fn new(rx: mpsc::Receiver<R>, tx: mpsc::Sender<S>) -> Self {
        Self { rx, tx }
    }

    pub async fn send(&mut self, response: S) -> MiasChannelResult<()> {
        self.tx
            .send(response)
            .await
            .map_err(|e| MiasChannelError::SendError(e.to_string()))
    }

    pub async fn recv(&mut self) -> MiasChannelResult<R> {
        self.rx.recv().await.ok_or(MiasChannelError::RecvError(
            "Failed to receive message".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_channel() {
        let (mut tx, mut rx) = channel::<i64, i64>(10);

        tx.send(10).await.unwrap();
        assert_eq!(rx.recv().await.unwrap(), 10);
        rx.send(20).await.unwrap();
        assert_eq!(tx.recv().await.unwrap(), 20);
    }
}
