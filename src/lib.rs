//! Executor-independent task yielding

use std::{future::Future, pin::Pin, task::Poll};

/// Yields execution to the `async` executor
pub fn yield_now() -> Yield {
    Yield(true)
}

pub struct Yield(bool);

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if self.0 {
            cx.waker().wake_by_ref();
            self.as_mut().0 = false;
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tokio_test() {
        use std::cell::Cell;
        let x = Cell::new(0u32);
        let a = async {
            for _ in 0..10000 {
                x.set(x.get() + 1);
                yield_now().await;
            }
        };
        let b = async {
            for i in 0..10000 {
                assert_eq!(x.get(), i + 1);
                tokio::task::yield_now().await;
            }
        };

        futures::future::join(a, b).await;
    }

    #[async_std::test]
    async fn async_std_test() {
        use std::cell::Cell;
        let x = Cell::new(0u32);
        let a = async {
            for _ in 0..10000 {
                x.set(x.get() + 1);
                yield_now().await;
            }
        };
        let b = async {
            for i in 0..10000 {
                assert_eq!(x.get(), i + 1);
                async_std::task::yield_now().await;
            }
        };

        futures::future::join(a, b).await;
    }
}
