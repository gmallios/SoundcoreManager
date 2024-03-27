import ReactDOM from 'react-dom/client';
import App from './App';
import './style.css';
import { appLogDir } from '@tauri-apps/api/path';
import { attachConsole } from 'tauri-plugin-log-api';
import { NextUIProvider } from '@nextui-org/react';

attachConsole();

(async () => {
  const logDir = await appLogDir();
  console.log(
    `Found a bug or something isn't working? \nCheck out the logs at ${logDir}\nand open/respond to an issue at https://github.com/gmallios/SoundcoreManager`
  );
})();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <NextUIProvider>
    <App />
  </NextUIProvider>
);
