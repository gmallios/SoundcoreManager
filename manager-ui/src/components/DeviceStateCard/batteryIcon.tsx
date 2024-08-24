import { SingleBattery } from '@generated-types/soundcore-lib';
import React from 'react';
import {
  BatteryCharging,
  BatteryFull,
  BatteryLow,
  BatteryMedium,
  BatteryWarning
} from 'lucide-react';

export const BatteryIcon: React.FC<{ battery: SingleBattery }> = ({ battery }) => {
  const { charging: isCharging, level } = battery;
  let icon = <BatteryWarning />;
  let colorClass = '';
  if (!isCharging) {
    switch (level) {
      case 0:
        icon = <BatteryWarning />;
        colorClass = 'text-danger';
        break;
      case 1:
      case 2:
        icon = <BatteryLow />;
        colorClass = 'text-warning';
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

  return <div className={'flex items-center gap-1 ' + colorClass}>{icon}</div>;
};
