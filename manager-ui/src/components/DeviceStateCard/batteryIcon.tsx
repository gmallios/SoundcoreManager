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
import React from 'react';
import { BatteryCharging, BatteryFull, BatteryLow, BatteryMedium, BatteryWarning } from 'lucide-react';

export const BatteryIcon: React.FC<{ battery: SingleBattery }> = ({ battery }) => {
  const { charging: isCharging, level } = battery;
  let icon = <BatteryWarning />;
  if (!isCharging) {
    switch (level) {
      case 0:
        icon = <BatteryWarning />;
        break;
      case 1:
      case 2:
        icon = <BatteryLow />;
        break;
      case 3:
      case 4:
        icon = <BatteryMedium />;
        break;
      case 5:
        icon = <BatteryFull />;
        break;
    }
  } else {
    icon = <BatteryCharging />;
  }

  return <div className={'flex items-center gap-1'}>{icon}</div>;
};
