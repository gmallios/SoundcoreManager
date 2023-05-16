import React, { FC } from "react";
import DeviceCard from "./DeviceCard";
import { Space, Stack } from "@mantine/core";

export const HomeScreen: FC = () => {

    return (
        <>
            <Stack p="md" justify="flex-start">
                <DeviceCard />
            </Stack>
        </>
    );
}