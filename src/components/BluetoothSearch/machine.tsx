import { createMachine } from "xstate";

const disconnectedScreenMachine = createMachine({
    tsTypes: {} as import("./machine.typegen").Typegen0,
    id: 'bluetooth_machine',
    initial: 'pending',
    states: {
        pending: {
            on: {
                SEARCH: 'searching'
            }
        },
        searching: {
            on: {
                REJECT: 'searchError',
                RESOLVE: 'searchSuccess'
            }
        },
        searchSuccess: {
            on: {
                SELECT_DEVICE: 'connecting'
            }
        },
        searchError: {
            on: {
                RETRY: 'searching'
            }
        },
        connecting: {
            on: {
                REJECT: 'pending',
                // RESOLVE: '#app.connected'
            }
        }
    }
});

export default disconnectedScreenMachine;