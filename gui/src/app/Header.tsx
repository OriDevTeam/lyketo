// Local Imports
import { VERSION } from "../App";

// External Imports
import {Code, Header, Text, useMantineColorScheme} from "@mantine/core";


export default function (): JSX.Element {
    const { colorScheme, toggleColorScheme } = useMantineColorScheme()

    return (
        <Header height={50} p="xs">
            <Text component="span" align="center" variant="gradient"
                  gradient={{ from: 'cyan', to: 'green', deg: 85 }} size="xl" weight={700}
                  style={{ fontFamily: 'Greycliff CF, sans-serif' }}
            >Lyketo GUI</Text>

            <Code sx={{ fontWeight: 700 }}>{VERSION}</Code>
        </Header>
    )

    /*
            <Group sx={{ height: '100%' }} px={20} position="apart">
                <ActionIcon variant="default" onClick={() => toggleColorScheme()} size={30}>
                    {colorScheme === 'dark' ? <IconSun size={16} /> : <IconMoonStars size={16} />}
                </ActionIcon>
            </Group>

     */
}


