pub fn launch_server() {
    #[cfg(debug_assertions)]
    let mut cmd = std::process::Command::new("killall")
        .arg("server")
        .spawn()
        .expect("failed to spawn killall command");
    let _res = cmd.wait().expect("failed to wait for killall command");


    use tauri::api::process::CommandEvent::{Stdout, Stderr, Error};
    let (mut rx, mut _tx) = tauri::api::process::Command::new_sidecar("server")
        .expect("failed to create server command")
        .spawn()
        .expect("failed to spawn server command");
        tauri::async_runtime::spawn(async move {
            // read events such as stdout
            while let Some(event) = rx.recv().await {
                match event {
                    Stdout(data) => {
                        log::debug!("{}", data);
                    }
                    Stderr(data) => {
                        log::debug!("{}", data);
                    }
                    Error(err) => {
                        log::debug!("{}", err);
                    }
                    _ => {},
                }
            }
        });
    std::thread::sleep(std::time::Duration::from_secs(1));
}