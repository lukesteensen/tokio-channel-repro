use futures::{task::Poll, Sink};
use std::{pin::Pin, task::Context};
use tokio::sync::mpsc;

pub struct Pipeline<T> {
    inner: mpsc::Sender<T>,
}

impl<T: Send + 'static> Sink<T> for Pipeline<T> {
    type Error = tokio::sync::mpsc::error::ClosedError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        self.inner.try_send(item).map_err(|e| panic!(e))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<T> Pipeline<T> {
    pub fn new(inner: mpsc::Sender<T>) -> Self {
        Self { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::Pipeline;
    use futures::{task::Poll, FutureExt, StreamExt};
    use std::{
        future,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    };

    #[tokio::test]
    async fn it_works() {
        let (tx_in, mut rx_in) = tokio::sync::mpsc::channel(1000);
        let (tx_out, rx_out) = tokio::sync::mpsc::channel(2000);
        let (trigger, signal) = tokio::sync::oneshot::channel();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter2 = Arc::clone(&counter);

        // Send input data
        let input_h = tokio::spawn(async move {
            let input = futures::stream::repeat(String::from("foo bar"));
            input.map(Ok).forward(Pipeline::new(tx_in)).await.ok();
        });

        // Accept input data until we get the shutdown signal, forwarding it and keeping track of
        // how many items we see
        let forward_h = tokio::spawn(async move {
            let mut shutdown = Some(signal);
            futures::stream::poll_fn(move |cx| {
                if let Some(s) = shutdown.as_mut() {
                    match s.poll_unpin(cx) {
                        Poll::Ready(_) => {
                            shutdown.take();
                            rx_in.close();
                        }
                        Poll::Pending => {}
                    }
                }

                rx_in.poll_next_unpin(cx)
            })
            .inspect(|_| {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .map(Ok)
            .forward(Pipeline::new(tx_out))
            .await
            .unwrap();

            counter.load(Ordering::SeqCst)
        });

        // Trigger shutdown after we've processed a reasonable number of events
        while counter2.load(Ordering::SeqCst) < 100 {
            tokio::time::delay_for(std::time::Duration::from_millis(1)).await;
        }
        trigger.send(()).unwrap();

        // Count how many events are successfully forwarded
        let count_h =
            tokio::spawn(async move { rx_out.fold(0usize, |acc, _| future::ready(acc + 1)).await });

        input_h.await.unwrap();
        let received_count = forward_h.await.unwrap();
        let forwarded_count = count_h.await.unwrap();

        assert_eq!(received_count, forwarded_count);
    }
}
