// Local Imports
import {Tab, TYPES} from "../Eter";
import {FILES} from "../../../modules/File";
import {hrefPath} from "./File";

// External Imports
import { useState } from 'react';
import { IconFiles, IconSearch} from "@tabler/icons";
import {
    createStyles, Table, ScrollArea, Chip, Text, Group, Checkbox,
    Button, Select, Container, TextInput
} from '@mantine/core';
import {filesize} from "filesize";


export default class implements Tab {
    name = "Disk Browser"
    color = "teal"
    icon = IconFiles
    component = component
}


function component(): JSX.Element {
    const data = FILES;

    return (
        <TableScrollArea data={data.data}></TableScrollArea>
    )
}


const useStyles = createStyles((theme) => ({
    header: {
        position: 'sticky',
        top: 0,
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
        transition: 'box-shadow 150ms ease',

        '&::after': {
            content: '""',
            position: 'absolute',
            left: 0,
            right: 0,
            bottom: 0,
            borderBottom: `1px solid ${
                theme.colorScheme === 'dark' ? theme.colors.dark[3] : theme.colors.gray[2]
            }`,
        },
    },

    scrolled: {
        boxShadow: theme.shadows.sm,
    },

    rowSelected: {
        backgroundColor:
            theme.colorScheme === 'dark'
                ? theme.fn.rgba(theme.colors[theme.primaryColor][7], 0.2)
                : theme.colors[theme.primaryColor][0],
    },

    lowbox: {
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[1],
        padding: theme.spacing.xs,
        borderRadius: theme.radius.md
    }
}))


interface TableScrollAreaProps {
    data: { name: string, size: string, type: number }[];
}


export function TableScrollArea({ data }: TableScrollAreaProps) {
    const { classes, cx } = useStyles();

    const [scrolled, setScrolled] = useState(false);
    const [selection, setSelection] = useState(['1']);

    const toggleRow = (id: string) =>
        setSelection((current) =>
            current.includes(id) ? current.filter((item) => item !== id) : [...current, id]
        );

    const toggleAll = () =>
        setSelection((current) => (current.length === data.length ?
            [] : data.map((item) => item.name)));

    const rows = data.map((row) =>  {
        const selected = selection.includes(row.name)

        const fileSize = filesize(row.size, {base: 2, standard: "jedec"}).toString()

        return (
            <tr key={row.name} className={cx({ [classes.rowSelected]: selected })}>
                <td>
                    <Checkbox
                        checked={selection.includes(row.name)}
                        onChange={() => toggleRow(row.name)}
                        transitionDuration={0}
                    />
                </td>

                <td>
                    <div>
                        <a href={hrefPath(row.hash)}>
                            <Text underline>{row.name}</Text>
                        </a>
                    </div>

                    {/*
                    <Link to={"/landing"} aria-multiline={true}>
                        {row.name}
                    </Link>
                    */}

                </td>

                <td>{fileSize}</td>

                <td>
                    <Chip disabled={true}>
                        <Text color={TYPES[row.type].color}>{TYPES[row.type].label}</Text>
                    </Chip>
                </td>
            </tr>
        )
    })

    const [search, setSearch] = useState('');

    return (
        <>
            <TextInput placeholder="Search file" mb="md" value={search}
                icon={<IconSearch size={14} stroke={1.5} />}
            />

            <ScrollArea sx={{ height: 415 }} onScrollPositionChange={({ y }) => setScrolled(y !== 0)}>

                <Table>
                    <thead className={cx(classes.header, { [classes.scrolled]: scrolled })}>

                    <tr>
                        <th style={{ width: 40 }}>
                            <Checkbox
                                onChange={toggleAll}
                                checked={selection.length === data.length}
                                indeterminate={selection.length > 0 && selection.length !== data.length}
                                transitionDuration={0}
                            />
                        </th>
                        <th>Name</th>
                        <th>Size</th>
                        <th>Type</th>
                        <th>Created at</th>
                        <th>Modified at</th>
                    </tr>

                    </thead>

                    <tbody>{rows}</tbody>
                </Table>
            </ScrollArea>


            <Container mt={15} size={"xl"} className={classes.lowbox}>

                <Group className={classes.lowbox}>
                    <Container size="xs" px="xs">
                        <Select label="Convert selected files to type:" placeholder="Pick a type" data={TYPES}/>

                        <Button color={"teal"} size={"xs"}>Convert</Button>
                    </Container>

                    <Container size="xs" px="xs">
                        <Text>Selected files operations</Text>
                        <Button color={"red"} size={"xs"}>Delete</Button>
                    </Container>
                </Group>

            </Container>
        </>

    )
}

