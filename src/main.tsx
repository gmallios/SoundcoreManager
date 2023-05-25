import React, { createContext } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./style.css";
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { MantineProvider, MantineTheme, MantineThemeOverride } from "@mantine/core";
import { BluetoothSearchScreen } from "./components/Search";
import { inspect } from "@xstate/inspect";
import { createActorContext, useInterpret, useMachine } from "@xstate/react";
import { InterpreterFrom, createMachine, interpret } from "xstate";
import globalMachine from "./components/Search/machine";


inspect({
  // url: 'https://statecharts.io/inspect',
  url: 'https://stately.ai/viz?inspect',
  iframe: false // open in new window
});import { appLogDir } from '@tauri-apps/api/path';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: "always",
      refetchIntervalInBackground: true,
    },
  },
});

const theme: MantineThemeOverride = {
  colorScheme: 'dark',
  primaryShade: 9,
  globalStyles(theme) {
    return {
      body: {
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.white,
      },
    };
  },
};


// export const ScreenManagerContext = createActorContext(screenManagerMachine, { devTools: true });



(async () => {
  const logDir = await appLogDir();
  console.log(`Found a bug or something isn't working? \nCheck out the logs at ${logDir}\nand open/respond to an issue at https://github.com/gmallios/SoundcoreManager`);
})();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <MantineProvider theme={theme} withGlobalStyles withNormalizeCSS>
    <QueryClientProvider client={queryClient}>
      {/* <ScreenManagerContext.Provider> */}
        <App />
      {/* </ScreenManagerContext.Provider> */}
      <ReactQueryDevtools />
    </QueryClientProvider>
  </MantineProvider>
);

