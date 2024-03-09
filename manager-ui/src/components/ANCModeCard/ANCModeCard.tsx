import { useDeviceModel } from '../../hooks/useSoundcoreDevice';
import { ANCModes } from '../../types/tauri-backend';
import BaseANCModeCard from './base';

export default function ANCModeCard() {
  const { data: model } = useDeviceModel();
  let Component = () => <div />;
  let ancButtons: Array<[string, ANCModes]> = [];
  let transButtons: Array<[string, ANCModes]> = [];
  const defaultComponent = () => {
    return <div>Not implemented</div>;
  };
  switch (model) {
    case 'A3935':
    case 'A3951':
      Component = BaseANCModeCard;
      ancButtons = [
        ['Transport', { mode: 'AncTransportMode' }],
        ['Outdoor', { mode: 'AncOutdoorMode' }],
        ['Indoor', { mode: 'AncIndoorMode' }],
        ['Custom', { mode: 'AncCustomValue', value: 0 }]
      ];
      transButtons = [
        ['Fully Trasparent', { mode: 'TransparencyFullyTransparentMode' }],
        ['Vocal Mode', { mode: 'TransparencyVocalMode' }]
      ];
      break;
    case 'A3027':
    case 'A3028':
    case 'A3029':
      Component = BaseANCModeCard;
      ancButtons = [
        ['Transport', { mode: 'AncTransportMode' }],
        ['Outdoor', { mode: 'AncOutdoorMode' }],
        ['Indoor', { mode: 'AncIndoorMode' }]
      ];
      transButtons = [['Fully Trasparent', { mode: 'TransparencyFullyTransparentMode' }]];
      break;
    default:
      Component = defaultComponent;
      break;
  }
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-expect-error
  return <Component ancModes={ancButtons} transModes={transButtons} />;
}
