## Manager Tauri App Architecture Overview
```mermaid
flowchart LR
        manager-ui  -->|tauri::command| manager-app
        manager-app -->|tauri event| manager-ui
        manager-app -->|mpsc| async-bridge
        async-bridge -->|mpsc| manager-app
        async-bridge <--> soundcore-lib
```
