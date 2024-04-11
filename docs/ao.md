## Manager Tauri App Architecture Overview
```mermaid
flowchart LR
        manager-ui  -->|tauri command| manager-app
        manager-app -->|tauri event| manager-ui
        manager-app -->|mpsc channel| async-bridge
        async-bridge -->|mpsc channel| manager-app
        async-bridge <--> soundcore-lib
```
