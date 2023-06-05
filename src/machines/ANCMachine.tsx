import { createMachine, assign } from "xstate";
import { ANCModes, SupportedANCProfiles } from "../types/tauri-backend";
import { useUpdateANC } from "../hooks/useSoundcoreDevice";
import { SupportedModels } from "../types/soundcore-lib";

const ancMutation = useUpdateANC();

interface ANCContext {
  anc_custom: number | null;
  model: SupportedModels | null;
  mode: SupportedANCProfiles | null;
}

type ANCEvent =
  | { type: "SET_MODE", mode: SupportedANCProfiles }
  | { type: "SET_ANC_CUSTOM", value: number }
  | { type: "SET_MODEL", model: string }
  | { type: "SET_NORMAL" }
  | { type: "SET_TRANSPARENCY" }
  | { type: "SET_NOISE_CANCELLING" }
  | { type: "SET_TRANS" }
  | { type: "SET_ANC" };


const hasTransMode = (ctx: ANCContext, event: ANCEvent) => {
  ctx.model === "A3951" ? true : false;
}

const ctxIsTransFull = (ctx: ANCContext, _event: ANCEvent) => {
  return ctx.mode === SupportedANCProfiles.TransparencyFullyTransparentMode;
}

const ctxIsTransVocal = (ctx: ANCContext, _event: ANCEvent) => {
  return ctx.mode === SupportedANCProfiles.TransparencyVocalMode;
}

const ctxIsAncIndoor = (ctx: ANCContext, _event: ANCEvent) => {
  return ctx.mode === SupportedANCProfiles.AncIndoorMode;
}

const ctxIsAncOutdoor = (ctx: ANCContext, _event: ANCEvent) => {
  return ctx.mode === SupportedANCProfiles.AncOutdoorMode;
}

const ctxIsAncTransport = (ctx: ANCContext, _event: ANCEvent) => {
  return ctx.mode === SupportedANCProfiles.AncTransportMode;
}

const ctxIsAncCustom = (ctx: ANCContext, _event: ANCEvent) => {
  return ctx.mode === SupportedANCProfiles.AncCustomValue;
}


const NoiseCancellingMachine = {
  noise_cancelling: {
    initial: "unknown",
    on: {
      SET_TRANS: "transparency",
      SET_NORMAL: "normal",
    },
    states: {
      unknown: {
        always: [
          { target: 'indoor', cond: ctxIsAncIndoor },
          { target: 'outdoor', cond: ctxIsAncOutdoor },
          { target: 'transport', cond: ctxIsAncTransport },
          { target: 'custom', cond: ctxIsAncCustom },
        ]
      },
      indoor: {},
      outdoor: {},
      transport: {},
      custom: {},
    },
  },
}

const TransparencyMachine = {
  transparency: {
    initial: "unknown",
    on: {
      SET_ANC: "noise_cancelling",
      SET_NORMAL: "normal",
    },
    states: {
      unknown: {
        always: [
          { target: 'full', cond: ctxIsTransFull },
          { target: 'vocal', cond: ctxIsTransVocal },
        ]
      },
      full: {},
      vocal: {},
    },
  },
};


const ANCProfiles = [
  SupportedANCProfiles.AncCustomValue, 
  SupportedANCProfiles.AncTransportMode, 
  SupportedANCProfiles.AncIndoorMode,
  SupportedANCProfiles.AncOutdoorMode,
]

const TransProfiles = [
  SupportedANCProfiles.TransparencyVocalMode,
  SupportedANCProfiles.TransparencyFullyTransparentMode,
];

const ANCMachine = createMachine<ANCContext, ANCEvent>(
  {
    /** @xstate-layout N4IgpgJg5mDOIC5QEEByBhAsgQwMYAsBLAOzAGIBlAUQBUB9NdO9AVQpoHlMBtABgF1EoAA4B7WIQAuhUcSEgAHogCMAZgAsAOlUBOAEz71AdnXL1AVlXKAbABoQAT0R6AHKs06XvC8pc7z6tYuRgC+IfaMOAQk5NT0mBwAIlR8gkggYhLSsvJKCKpGepqmGsrmRkZBekaq9k4Iru6e3uq8LtbWyjre1mERGFFEpJokUmSp8plSMnLpeXq8yppm1kbm3l0FvOZ2johG+ppGvLzdO0Y23ap9IJF4Q2AjxGPcymki4tM5cyo6WsbqYKWVQLUzqPR1fZ6ayaarHGxBdTqHSrG53aLDUaScZ6d4ZT7ZWagPK+dyqXStax6akuLwFSEIFw7WHqVRUlxqcxmWlogb3GKaYiiABOAFtsAAbSi0BgYCbpKaE3KIVpLHZI9U6bocnQMqnmTSrLlImzKPTmHShcK3PkYx5CsWS6X0GgAJTQFHlHyyM2VCHKRk0LlZ1X8Aa66gZwSKgTaRmD7TW1N5WH5wyFhFgYDouGwxFwYAlEpIUGddDdHq9+J932JiGCWhR1ib1h2HORUcKxWsp2pPdU7XMKcGAozWZzeYLRZLZdQHFdmGQABkq4rfT9-S4isplEm2TpVFzKgyajDT7peNYNNUDMO0-bRJns7n84Xi8QoJoAK7EADWQoAd2IcYBEmAl1zrBAdDUZYeiCS8yl3cw9WqYpzVNXgrA6fQ7ztQVH3HF8p3fT8f3-UQgPGN4wJrIlFEQaD3GUOCvE6I9kL2BAggNfQuXWIILisPRcIefCnwnV9pw-b8-0A4DuFxGivjovJGNgwJ4LYpCGTbDx0MEyoFguETRwI59JzfEsZPIyjuFUPE11reioJg5iNNYxC1hPJE0ONTDwQtKwTOGSRhTzWBhGwYUwHzBwy0YVdwKckkTh0TQ2lUVL9GpA8OPqaNu17aFMMHZRgseULwsi6LYtnedFxXUCFSSlTEAHA1Wh1KlW2bRYGWYqxDUtNkmWg5EdHKzRKuICKopi3AHGsuSQIclq-Wy4pKUCBEew5frMPcJEaiZbbtXUSbptmmqFqWij5Oo5raPW6lNsvbaOl25R9ugjwAS6PQNEBckwmtIUIDgeR0QeJSlQ3XhDUqCxtk63xjD8BldG3NYyivQoQ14K1+lTPCsRhiDnMBBHrCRgI2lRlwT0vdLm3KVsByTCbrSh0zHQlMnkralFYUvakCkBYN9BPbpinWaC2gtNwTEmsdzMkkj+dahATBcdKzWMVszWpAGGWMNKjTMNpgnjDllbMiTiKssi5I1v0DhhRY9H1rkjeNzj9QRrlaWqRXzVt8SiMs6SSAgUQRRdjdmIODxqcvHp4zclCDXNjo9C6DkqTDwiLKkz9RC-SQY7jx7lL9RO0pRSk045bw9WDZZsfMcwgWD64udtUSVftyPP0usRhUkePILr5PG8CdOW848526PHrWQCc6++Jge7YjkvNFwL9YEkURRUn5zp4b1O5+byNOJMIpzYsSwCdVC6wpm6r5vqb0a43NZ3GCByLuBMuQgghJxP4Wgug1BBGNA8rg35VTmrFW6QEz55C8EsQBZQvDYzAftAcywLadXaPoFwiCP7IJugAMy-EWdBKg2hpUyicGwxpAa7HqANJYzZTyjTMFqChV0v6aAAG6iFzHzausMp5MO0CcRYBtWRIk4Yw6W0DWgAxTjUK0YQgA */
    id: "ANCMachine",
    initial: "init",
    context: {
      mode: null,
      anc_custom: null,
      model: null
    },
    on: {
      SET_ANC_CUSTOM: {
        actions: assign({
          anc_custom: (_ctx: ANCContext, event: ANCEvent) => 'value' in event ? event.value : 0
        }), 
      },
      SET_MODE: {
        actions: assign({
          mode: (_ctx: ANCContext, event: ANCEvent) => 'mode' in event ? event.mode : null
        }),
      },
    },
    states: {
      init: {
        always: [
          { target: 'normal', cond: 'ctxIsNormal' },
          { target: 'noise_cancelling', cond: 'ctxIsAnc' },
          { target: 'transparency', cond: 'ctxIsTrans' },
        ],
      },
      normal: {
        on: {
          SET_ANC: "noise_cancelling",
          SET_TRANS: "transparency",
        },
      },
      ...NoiseCancellingMachine,
      ...TransparencyMachine
    }
  },
  {
    guards: {
      ctxIsNormal: (ctx, _event) => {
        return ctx.mode === SupportedANCProfiles.Normal;
      },
      ctxIsAnc: (ctx, _event,) => {
        return ANCProfiles.includes(ctx.mode!);
      },
      ctxIsTrans: (ctx, _event) => {
        return TransProfiles.includes(ctx.mode!);
      },
    }
  }
);



