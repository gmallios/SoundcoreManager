use std::future::Future;
use std::time::Duration;

use tokio::task::JoinHandle;

use crate::{ManagerFuture, ManagerJoinHandle};

pub struct TokioFuture;

impl ManagerFuture for TokioFuture {
    type JoinHandle = TokioJoinHandle;

    fn spawn<F, O>(f: F) -> Self::JoinHandle
    where
        F: Future<Output = O> + Send + 'static,
        O: Send + 'static,
    {
        TokioJoinHandle {
            handle: tokio::task::spawn(async move {
                f.await;
            }),
        }
    }

    fn spawn_local(f: impl Future + 'static) -> Self::JoinHandle {
        TokioJoinHandle {
            handle: tokio::task::spawn_local(async move {
                f.await;
            }),
        }
    }

    async fn sleep(dur: Duration) {
        tokio::time::sleep(dur).await;
    }

    async fn timeout<F, T>(dur: Duration, fut: F) -> Result<T, ()>
    where
        F: Future<Output = T>,
    {
        match tokio::time::timeout(dur, fut).await {
            Ok(res) => Ok(res),
            Err(_) => Err(()),
        }
    }
}

pub struct TokioJoinHandle {
    handle: JoinHandle<()>,
}

impl ManagerJoinHandle for TokioJoinHandle {
    fn abort(&self) {
        self.handle.abort();
    }
}
