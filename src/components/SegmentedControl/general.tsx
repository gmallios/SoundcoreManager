import { Center, createStyles, rem, SegmentedControl, SegmentedControlItem } from "@mantine/core";
import { FC, useEffect, useState } from "react"


import ANCIcon from "../../assets/ambient_icon_anc.png";
import NormalIcon from "../../assets/ambient_icon_off.png";
import TransIcon from "../../assets/ambient_icon_trans.png";
import { useANC } from "../../hooks/useSoundcoreDevice";


export interface GeneralANCSegmentedControlProps {
    defaultValue: GeneralANCSegmentedControlValues;
    onChange: (value: string) => void;
}

export enum GeneralANCSegmentedControlValues {
    ANC = 'ANC',
    NORMAL = 'NORMAL',
    TRANSPARENCY = 'TRANSPARENCY'
}

export const GeneralANCSegmentedControl: FC<GeneralANCSegmentedControlProps> = (props: GeneralANCSegmentedControlProps) => {
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

    const GeneralSegmentedControlData: SegmentedControlItem[] = [
        {
            value: GeneralANCSegmentedControlValues.ANC,
            label: (
                <Center>
                    <img src={ANCIcon} height={30} />
                </Center>
            )
        },
        {
            value: GeneralANCSegmentedControlValues.NORMAL,
            label: (
                <Center>
                    <img src={NormalIcon} height={30} />
                </Center>
            )
        },
        {
            value: GeneralANCSegmentedControlValues.TRANSPARENCY,
            label: (
                <Center>
                    <img src={TransIcon} height={30} />
                </Center>
            )
        }
    ];

    const { data: currentANCMode, isSuccess } = useANC();
    const { classes: segmentedControlClasses } = useSegmentedControlStyles();
    const [value, setValue] = useState(props.defaultValue);

    useEffect(() => {
        console.log(currentANCMode);
    }, [currentANCMode]);

    return (
        <>
            <SegmentedControl
                fullWidth
                radius="xl"
                size="sm"
                data={GeneralSegmentedControlData}
                classNames={segmentedControlClasses}
                value={value}
                onChange={(value: GeneralANCSegmentedControlValues) => {
                    setValue(value);
                    props.onChange(value)
                }}
            />
        </>
    )
}