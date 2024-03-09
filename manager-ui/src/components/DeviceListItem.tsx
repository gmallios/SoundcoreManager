import { ListItem, ListItemButton, ListItemIcon, ListItemText } from '@mui/material';
import BluetoothIcon from '@mui/icons-material/Bluetooth';
import DoneIcon from '@mui/icons-material/Done';

export interface IDeviceListItemProps {
  name: string;
  isConnected: boolean;
  isSelected: boolean;
  idx: number;
  onItemClicked: (event: React.MouseEvent<HTMLDivElement, MouseEvent>, index: number) => void;
}

export default function DeviceListItem(props: IDeviceListItemProps) {
  const { name, isConnected, isSelected, onItemClicked, idx } = props;

  return (
    <ListItem disablePadding>
      <ListItemButton
        selected={isSelected}
        onClick={(event) => {
          onItemClicked(event, idx);
        }}>
        <ListItemIcon>
          <BluetoothIcon />
        </ListItemIcon>
        <ListItemText primary={name} />
        {isConnected && (
          <ListItemIcon>
            <DoneIcon />
          </ListItemIcon>
        )}
      </ListItemButton>
    </ListItem>
  );
}
