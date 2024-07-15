use futures::pin_mut;
use js_sys::Promise;
use std::{future::Future, rc::Rc, time::Duration};
use tokio::{select, sync::Notify};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use crate::{ManagerFuture, ManagerJoinHandle};

pub struct WasmFuture;

impl ManagerFuture for WasmFuture {
    type JoinHandle = WasmJoinHandle;

    fn spawn<F, O>(f: F) -> Self::JoinHandle
    where
        F: Future<Output = O> + Send + 'static,
        O: Send + 'static,
    {
        Self::spawn_local(f)
    }

    fn spawn_local(fut: impl Future + 'static) -> Self::JoinHandle {
        let handle = WasmJoinHandle {
            quit: Rc::new(Default::default()),
        };

        let quit = handle.quit.clone();

        wasm_bindgen_futures::spawn_local(async move {
            select! {
                _ = fut => (),
                _ = quit.notified() => (),
            }
        });
        handle
    }

    async fn sleep(dur: Duration) {
        JsFuture::from(Promise::new(&mut move |resolve, _reject| {
            window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &resolve,
                    dur.as_millis() as i32,
                )
                .unwrap();
        }))
        .await
        .unwrap();
    }

    async fn timeout<F, T>(dur: Duration, fut: F) -> Result<T, ()>
    where
        F: Future<Output = T>,
    {
        let sleep_future = JsFuture::from(Promise::new(&mut move |resolve, _reject| {
            window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &resolve,
                    dur.as_millis() as i32,
                )
                .unwrap();
        }));

        pin_mut!(fut);
        pin_mut!(sleep_future);

        select! {
            result = fut => Ok(result),
            _ = sleep_future => Err(()),
        }
    }
}

pub struct WasmJoinHandle {
    quit: Rc<Notify>,
}

impl ManagerJoinHandle for WasmJoinHandle {
    fn abort(&self) {
        self.quit.notify_one();
    }
}
