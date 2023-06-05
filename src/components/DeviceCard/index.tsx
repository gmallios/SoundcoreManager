import { Text, Card, Group, createStyles, rem, RingProgress, Stack, SegmentedControl, Grid, SimpleGrid, Collapse, Center, Box, SegmentedControlItem, Progress } from '@mantine/core';
import { FC, useEffect, useState } from 'react';
import useDeviceImage from '../../hooks/useDeviceImage';
import { useBatteryLevel, useCharging, useDeviceModel, useStatus } from '../../hooks/useSoundcoreDevice';
import useGlobalStore from '../../hooks/useGlobalStore';
import { shallow } from 'zustand/shallow';
import { useDisclosure } from '@mantine/hooks';
import { GeneralANCSegmentedControl } from '../SegmentedControl';
import { GeneralANCSegmentedControlValues } from '../SegmentedControl/general';


const useStyles = createStyles((theme) => ({
    card: {
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
    },
    leftCol: {
        paddingLeft: 0,
        paddingRight: 0,
    },
    rightCol: {
        paddingLeft: 0,
    },
    sideStack: {
        width: '100%',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        alignItems: 'center',
        padding: 0,
        paddingRight: 0,
    },
    deviceWrapper: {
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        margin: 0,
        padding: 0,

    },
    deviceImage: {
        filter: 'drop-shadow(10px 10px 7px rgba(0,0,0,0.5))',
    },
    deviceName: {
        color: theme.colorScheme === 'dark' ? theme.colors.dark[1] : theme.black,
        fontWeight: 400,
        marginTop: -10,
    },
    deviceInfo: {
        display: 'flex',
        flex: 1,
        width: '100%',
    },
    batteryProgress: {
        width: rem(75),
        height: rem(6),
    }
}));

const useSegmentedControlStyles = createStyles((theme) => ({
    root: {
        width: '100%',
        marginTop: 'auto',
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.white,
        boxShadow: theme.shadows.md,
        border: `${rem(1)} solid ${theme.colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[1]}`,
    },

    indicator: {
        backgroundImage: theme.fn.gradient({ from: 'pink', to: 'yellow' }),
    },

    control: {
        border: '0 !important',
    },

    label: {
        '&, &:hover': {
            '&[data-active]': {
                color: theme.colors.white,
            },
        },
    },
}));



interface DeviceCardProps {
}

const DeviceCard: FC<DeviceCardProps> = (props: DeviceCardProps) => {
    const { classes } = useStyles();
    const { classes: segmentedControlClasses } = useSegmentedControlStyles();
    const { isLoading: isChargingLoading, data: chargingData, isError: isChargingError } = useCharging();
    const { isLoading: isBatteryLoading, data: batteryData, isError: isBatteryError } = useBatteryLevel();
    const { data: deviceModel, isLoading: isDeviceModelLoading } = useDeviceModel();
    const btDevice = useGlobalStore((state) => state.btDevice, shallow);

    console.log(batteryData);

    const [currentAncMode, setCurrentAncMode] = useState('NORMAL');
    const [opened, { open, close }] = useDisclosure(false);

    useEffect(() => {
        currentAncMode === GeneralANCSegmentedControlValues.NORMAL ? close() : open();
    }, [currentAncMode]);

    return (
        <>
            <Card withBorder p="xs" radius="lg" className={classes.card}>
                <Grid columns={3}>
                    <Grid.Col span={1} className={classes.leftCol}>
                        <Stack p="xs" className={classes.sideStack} style={{ paddingRight: 0 }}>
                            {useDeviceImage({ device: deviceModel!, class: classes.deviceImage })}
                            <Text fz="lg" className={classes.deviceName}>
                                {removeMfgName(btDevice!.name)}<Text span fz="md">'s</Text>
                            </Text>
                        </Stack>
                    </Grid.Col>
                    <Grid.Col span={2} className={classes.rightCol}>
                        <Stack p="xs" className={classes.sideStack} style={{ paddingLeft: 0 }}>
                            <div className={classes.deviceInfo}>
                                Device Info
                                <Progress radius="xl" value={50} className={classes.batteryProgress} />
                            </div>
                            <GeneralANCSegmentedControl
                                value={GeneralANCSegmentedControlValues.ANC}
                                onChange={(val) => setCurrentAncMode(val)}
                            />
                        </Stack>
                    </Grid.Col>
                </Grid>
                <Collapse in={opened} transitionDuration={320} >
                    <SegmentedControl
                        fullWidth
                        radius="lg"
                        size="xs"
                        classNames={segmentedControlClasses}
                        data={['Transport', 'Indoor', 'Outdoor', 'Custom']}
                    />
                </Collapse>
            </Card>
        </>
    )
}

const removeMfgName = (name: string) => {
    return name.replace('Soundcore ', '');
}


export default DeviceCard;