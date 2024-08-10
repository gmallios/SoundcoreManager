// import BatteryAlertIcon from '@mui/icons-material/BatteryAlert';
// import Battery20Icon from '@mui/icons-material/Battery20';
// import BatteryCharging20Icon from '@mui/icons-material/BatteryCharging20';
// import Battery30Icon from '@mui/icons-material/Battery30';
// import BatteryCharging30Icon from '@mui/icons-material/BatteryCharging30';
// import Battery60Icon from '@mui/icons-material/Battery60';
// import BatteryCharging60Icon from '@mui/icons-material/BatteryCharging60';
// import Battery80Icon from '@mui/icons-material/Battery80';
// import BatteryCharging80Icon from '@mui/icons-material/BatteryCharging80';
// import BatteryFullIcon from '@mui/icons-material/BatteryFull';
// import BatteryChargingFullIcon from '@mui/icons-material/BatteryChargingFull';
// import BatteryUnknownIcon from '@mui/icons-material/BatteryUnknown';
import { SingleBattery } from '@generated-types/soundcore-lib';

export const BatteryIcon: React.FC<{ battery: SingleBattery }> = ({ battery }) => {
  const { charging: isCharging, level } = battery;
  const icon = null;

  // Reported battery level ranges between 0-5
  // switch (level) {
  //   case 0:
  //     icon = !isCharging ? <BatteryAlertIcon rotate={90} /> : <BatteryCharging20Icon />;
  //     break;
  //   case 1:
  //     icon = !isCharging ? <Battery20Icon rotate={90} /> : <BatteryCharging20Icon />;
  //     break;
  //   case 2:
  //     icon = !isCharging ? <Battery30Icon rotate={90} /> : <BatteryCharging30Icon />;
  //     break;
  //   case 3:
  //     icon = !isCharging ? <Battery60Icon rotate={90} /> : <BatteryCharging60Icon />;
  //     break;
  //   case 4:
  //     icon = !isCharging ? <Battery80Icon rotate={90} /> : <BatteryCharging80Icon />;
  //     break;
  //   case 5:
  //     icon = !isCharging ? <BatteryFullIcon rotate={90} /> : <BatteryChargingFullIcon />;
  //     break;
  //   default:
  //     icon = <BatteryUnknownIcon />;
  //     break;
  // }

  return (
    <div className={'flex items-center gap-1'}>
      <p className={'text-small text-foreground/80'}>{level * 2 * 10}%</p>
      <div className={'rotate-90'}>{icon}</div>
    </div>
  );
};
