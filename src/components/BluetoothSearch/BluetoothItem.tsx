import React from "react";
import { Container, Divider, Grid, rem, UnstyledButton, createStyles, Image, Text, Card, Skeleton } from "@mantine/core";
import { getSoundcoreIcon } from "../../utils";
import { SupportedModels } from "../../types/soundcore-lib";

const transitionSpeed = "0.2s";
const useStyles = createStyles((theme) => ({
    card: {
        transition: `background-color ${transitionSpeed} ease-in-out, transform 100ms ease`,
        backgroundColor: theme.colors.dark[7],
        marginLeft: rem(10),
        marginRight: rem(10),
        ":hover": {
            backgroundColor: theme.colors.dark[8],
            transform: 'scale(1.01)',
        },
        "&:hover .cardImg": {
            transform: "translateY(-5px)",
            transition: `transform ${transitionSpeed} ease-in-out`,
        },
        "&:hover .cardTitle": {
            color: theme.colors.red[4],
            transition: `color ${transitionSpeed} 0.1s ease-in-out`,
        }
    },
    cardImgContainer: {
        marginLeft: "auto",
    },
    cardImg: {
        transition: `transform ${transitionSpeed} ease-in-out`,
        WebkitFilter: "drop-shadow(12px 12px 7px rgba(0, 0, 0, 0.5)) brightness(1.05)",
    },
    cardTitle: {
        marginRight: "auto",
        transition: `color ${transitionSpeed} ease-in-out`,
    },
    cardInner: {
        display: "flex",
        justifyContent: "space-between",
        flexDirection: "row",
        alignItems: "center",
    }
}));


export default function BluetoothItem(props: BluetoothItemProps) {
    const { classes, cx } = useStyles();
    
    return (
        <Card withBorder radius="md" p="md" className={cx(classes.card)} onClick={(event) => props.onItemClicked(event, props.idx)}>
            <Container className={cx(classes.cardInner)}>
                <div className={cx(classes.cardTitle, "cardTitle")}>
                    <Text fz="md" fw={500}>
                        {props.name}
                    </Text>
                    <Text fz="sm" fw={500} c="dimmed">
                        {props.model}
                    </Text>
                </div>
                <div className={cx(classes.cardImgContainer)}>
                    <Image className={cx(classes.cardImg, "cardImg")} height={90} fit="contain" src={getSoundcoreIcon(props.model)} />
                </div>
            </Container>
        </Card >
    );
}


export function BluetoothItemSkeleton() {
    const { classes, cx } = useStyles();
    
    return (
        <Card withBorder radius="md" p="md" className={cx(classes.card)}>
            <Container className={cx(classes.cardInner)}>
                <div className={cx(classes.cardTitle, "cardTitle")}>
                    <Skeleton height={13} width={200} radius="xl" />
                    <Skeleton height={10} width={60} mt={6} radius="xl" />
                </div>
                <div className={cx(classes.cardImgContainer)}>
                    <Skeleton height={90} width={90} radius="md" />
                </div>
            </Container>
        </Card >
    );
}


export interface BluetoothItemProps {
    idx: number;
    name: string;
    isConnected: boolean;
    model: SupportedModels;
    onItemClicked: (event: React.MouseEvent<HTMLDivElement, MouseEvent>, index: number) => void;
}