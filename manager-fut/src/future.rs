use std::time::Duration;

use futures::Future;

pub trait ManagerFuture {
    type JoinHandle: ManagerJoinHandle;

    fn spawn<F, O>(f: F) -> Self::JoinHandle
    where
        F: Future<Output = O> + Send + 'static,
        O: Send + 'static;

    fn spawn_local(fut: impl Future + 'static) -> Self::JoinHandle;
    async fn sleep(dur: Duration);
    async fn timeout<F, T>(dur: Duration, fut: F) -> Result<T, ()>
    where
        F: Future<Output = T>;
}

pub trait ManagerJoinHandle {
    fn abort(&self);
}
