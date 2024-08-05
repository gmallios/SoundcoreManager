import ReactDOM from 'react-dom/client';
import TauriApp from './TauriApp';
import './style.css';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { appLogDir } from '@tauri-apps/api/path';
import { WebApp } from './WebApp';
import { attachConsole } from 'tauri-plugin-log-api';
import { NextUIProvider } from '@nextui-org/react';

declare global {
  interface Window {
    isTauri: boolean;
  }
}
window.isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;

const darkTheme = createTheme({
  palette: {
    mode: 'dark'
  }
});

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: 'always',
      refetchIntervalInBackground: true
    }
  }
});

if (window.isTauri) {
  // Tauri-specific setup
  const _ = attachConsole();
  (async () => {
    const logDir = await appLogDir();
    console.log(
      `Found a bug or something isn't working? \nCheck out the logs at ${logDir}\nand open/respond to an issue at https://github.com/gmallios/SoundcoreManager`
    );
  })();
}

if (window.isTauri) {
  ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <ThemeProvider theme={darkTheme}>
      <QueryClientProvider client={queryClient}>
        <TauriApp />
      </QueryClientProvider>
    </ThemeProvider>);
} else {
  ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <NextUIProvider>
      <main className="dark text-foreground default-bg h-screen overflow-auto">
        <WebApp />
      </main>
    </NextUIProvider>
  );
}

