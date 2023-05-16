import { Text, Card, Group, createStyles, rem, RingProgress, Stack } from '@mantine/core';
import { FC } from 'react';
import useDeviceImage from '../../hooks/useDeviceImage';
import { useBatteryLevel, useCharging, useDeviceModel } from '../../hooks/useSoundcoreDevice';

const useStyles = createStyles((theme) => ({
    card: {
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
    },

    productName: {
        // // fontFamily: `Greycliff CF, ${theme.fontFamily}`,
        // fontFamily: `Helvetica, ${theme.fontFamily}`,
        fontWeight: 400,
        // lineHeight: 0,
        marginTop: -7
    },

    lead: {
        fontFamily: `Greycliff CF, ${theme.fontFamily}`,
        fontWeight: 700,
        fontSize: rem(22),
        lineHeight: 1,
    },

    inner: {
        display: 'flex',

        [theme.fn.smallerThan('xs')]: {
            flexDirection: 'column',
        },
    },

    deviceImage: {
        flex: 1,
        display: 'flex',
        justifyContent: 'flex-end',
        alignItems: 'center',

        [theme.fn.smallerThan('xs')]: {
            justifyContent: 'center',
            marginTop: theme.spacing.md,
        },
    },

    stack: {
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
    }
}));

interface DeviceCardProps {
}

const DeviceCard: FC<DeviceCardProps> = (props: DeviceCardProps) => {
    const { classes, theme } = useStyles();
    const { isLoading: isChargingLoading, data: chargingData, isError: isChargingError } = useCharging();
    const { isLoading: isBatteryLoading, data: batteryData, isError: isBatteryError } = useBatteryLevel();
    const { data: deviceModel, isLoading: isDeviceModelLoading } = useDeviceModel();

    return (
        <>
            {/* <Card withBorder p="md" radius="md" className={classes.card}> */}
                <Stack className={classes.stack} p="xs">
                    <div>
                        {useDeviceImage(deviceModel!)}
                        <Text fz="xl" className={classes.productName}>
                            Liberty Air 2 Pro's
                        </Text>
                    </div>
                    
                </Stack>
                {/* <div className={classes.inner}>
                    <div>
                        <Text fz="xl" className={classes.productName}>
                            Soundcore Liberty Air 2 Pro
                        </Text>
                        <div>
                            <Text className={classes.lead} mt={30}>
                                completed
                            </Text>
                            <Text fz="xs" color="dimmed">
                                Completed
                            </Text>
                        </div>
                        <Group mt="lg">
                            <div>
                                <Text className={classes.productName}>Value</Text>
                                <Text size="xs" color="dimmed">
                                    Label
                                </Text>
                            </div>
                        </Group>
                    </div>

                    <div className={classes.deviceImage}>
                        {useDeviceImage(deviceModel!)}
                    </div>
                </div> */}
            {/* </Card> */}
            {/* <Card withBorder p="md" radius="md" className={classes.card}>
                Example
            </Card> */}
        </>
    )
}


export default DeviceCard;