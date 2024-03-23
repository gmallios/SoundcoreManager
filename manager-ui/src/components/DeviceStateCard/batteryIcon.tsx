import BatteryAlertIcon from '@mui/icons-material/BatteryAlert';
import Battery20Icon from '@mui/icons-material/Battery20';
import BatteryCharging20Icon from '@mui/icons-material/BatteryCharging20';
import Battery30Icon from '@mui/icons-material/Battery30';
import BatteryCharging30Icon from '@mui/icons-material/BatteryCharging30';
import Battery60Icon from '@mui/icons-material/Battery60';
import BatteryCharging60Icon from '@mui/icons-material/BatteryCharging60';
import Battery80Icon from '@mui/icons-material/Battery80';
import BatteryCharging80Icon from '@mui/icons-material/BatteryCharging80';
import BatteryFullIcon from '@mui/icons-material/BatteryFull';
import BatteryChargingFullIcon from '@mui/icons-material/BatteryChargingFull';
import BatteryUnknownIcon from '@mui/icons-material/BatteryUnknown';
import { Grid, Typography } from '@mui/material';
import { SingleBattery } from '@generated-types/soundcore-lib';

export const BatteryIcon: React.FC<{ battery: SingleBattery }> = ({ battery }) => {
  const { charging: isCharging, level } = battery;
  let icon = null;

  // Reported battery level ranges between 0-5
  switch (level) {
    case 0:
      icon = !isCharging ? <BatteryAlertIcon /> : <BatteryCharging20Icon />;
      break;
    case 1:
      icon = !isCharging ? <Battery20Icon /> : <BatteryCharging20Icon />;
      break;
    case 2:
      icon = !isCharging ? <Battery30Icon /> : <BatteryCharging30Icon />;
      break;
    case 3:
      icon = !isCharging ? <Battery60Icon /> : <BatteryCharging60Icon />;
      break;
    case 4:
      icon = !isCharging ? <Battery80Icon /> : <BatteryCharging80Icon />;
      break;
    case 5:
      icon = !isCharging ? <BatteryFullIcon /> : <BatteryChargingFullIcon />;
      break;
    default:
      icon = <BatteryUnknownIcon />;
      break;
  }

  return (
    <Grid item>
      {icon}
      <Typography variant="body2" color="text.secondary">
        {level * 2 * 10}%
      </Typography>
    </Grid>
  );
};
