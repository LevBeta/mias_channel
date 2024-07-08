use tokio::sync::{mpsc, oneshot};

use crate::error::{MiasChannelError, MiasChannelResult};

pub type ResponderMessage<R, S> = (R, oneshot::Sender<S>);

pub fn responder_channel<R, S>(
    buffer: usize,
) -> (
    ResponderSender<R, S>,
    mpsc::Receiver<ResponderMessage<R, S>>,
) {
    let (tx, rx) = mpsc::channel::<ResponderMessage<R, S>>(buffer);
    (ResponderSender::new(tx), rx)
}

/// The sender side of the responder channel.
pub struct ResponderSender<R, S> {
    pub(crate) tx: mpsc::Sender<ResponderMessage<R, S>>,
}

impl<R, S> ResponderSender<R, S> {
    pub(crate) fn new(tx: mpsc::Sender<ResponderMessage<R, S>>) -> Self {
        Self { tx }
    }

    pub async fn send(&self, request: R) -> MiasChannelResult<S> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send((request, tx))
            .await
            .map_err(|e| MiasChannelError::SendError(e.to_string()))?;
        Ok(rx
            .await
            .map_err(|e| MiasChannelError::RecvError(e.to_string()))?)
    }
}

#[cfg(test)]
mod responder_tests {
    use super::*;

    #[tokio::test]
    async fn test_responder_channel() {
        let (tx, mut rx) = responder_channel::<i64, f64>(10);

        tokio::task::spawn(async move {
            while let Some((req, tx)) = rx.recv().await {
                tx.send(req as f64).unwrap();
            }
        });

        let res = tx.send(10).await;

        assert_eq!(res.unwrap(), 10.0);
    }
}
