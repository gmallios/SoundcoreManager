import { SoundcoreDeviceState } from '@generated-types/soundcore-lib';
import {
  Modal,
  ModalBody,
  ModalContent,
  ModalHeader,
  Table,
  TableBody,
  TableCell,
  TableColumn,
  TableHeader,
  TableRow
} from '@nextui-org/react';

export interface DeviceInfoModalProps {
  isOpen: boolean;
  onClose: () => void;
  state: SoundcoreDeviceState;
}

export const DeviceInfoModal: React.FC<DeviceInfoModalProps> = ({ isOpen, onClose, state }) => {
  return (
    <Modal backdrop="blur" isOpen={isOpen} onClose={onClose}>
      <ModalContent>
        <ModalHeader>Device Information</ModalHeader>
        <ModalBody className="p-0">
          <Table hideHeader removeWrapper className="p-4">
            <TableHeader>
              <TableColumn>Hidden</TableColumn>
              <TableColumn>Hidden</TableColumn>
            </TableHeader>
            <TableBody>
              <TableRow key="1">
                <TableCell>Model</TableCell>
                <TableCell>{state.serial?.model}</TableCell>
              </TableRow>
              <TableRow key="2">
                <TableCell>Serial Number</TableCell>
                <TableCell>{state.serial?.value}</TableCell>
              </TableRow>
              <TableRow key="3">
                <TableCell>Firmware Version</TableCell>
                <TableCell>
                  {state.fw?.primary.major}.{state.fw?.primary.minor}
                </TableCell>
              </TableRow>
              <TableRow key="4">
                <TableCell>LDAC</TableCell>
                <TableCell>{state.ldac ? 'Enabled' : 'Disabled/Unknown'}</TableCell>
              </TableRow>
              <TableRow key="5">
                <TableCell>Prompt Language</TableCell>
                <TableCell>{state.promptLanguage}</TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </ModalBody>
      </ModalContent>
    </Modal>
  );
};
