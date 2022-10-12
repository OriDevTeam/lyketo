// Local Imports
import {Section, VIEWS} from "./Views";
import {VERSION} from "../App";

// External Imports
import {useState} from 'react';
import {Code, createStyles, Group, Navbar, Text} from '@mantine/core';
import {useNavigate} from "react-router-dom";


const useStyles = createStyles((theme, _params, getRef) => {
    const icon = getRef('icon');
    return {
        header: {
            paddingBottom: theme.spacing.md,
            marginBottom: theme.spacing.md * 1.5,
            borderBottom: `1px solid ${
                theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
            }`,
        },

        footer: {
            paddingTop: theme.spacing.md,
            marginTop: theme.spacing.md,
            borderTop: `1px solid ${
                theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
            }`,
        },

        link: {
            ...theme.fn.focusStyles(),
            display: 'flex',
            alignItems: 'center',
            textDecoration: 'none',
            fontSize: theme.fontSizes.sm,
            color: theme.colorScheme === 'dark' ? theme.colors.dark[1] : theme.colors.gray[7],
            padding: `${theme.spacing.xs}px ${theme.spacing.sm}px`,
            borderRadius: theme.radius.sm,
            fontWeight: 500,

            '&:hover': {
                backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[0],
                color: theme.colorScheme === 'dark' ? theme.white : theme.black,

                [`& .${icon}`]: {
                    color: theme.colorScheme === 'dark' ? theme.white : theme.black,
                },
            },
        },

        linkIcon: {
            ref: icon,
            color: theme.colorScheme === 'dark' ? theme.colors.dark[2] : theme.colors.gray[6],
            marginRight: theme.spacing.sm,
        },

        linkActive: {
            '&, &:hover': {
                backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor })
                    .background,
                color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color,
                [`& .${icon}`]: {
                    color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color,
                },
            },
        },

        slogan: {
            display: "flow"
        }
    };
});



export default function (): JSX.Element {
    const { classes, cx } = useStyles();
    const [ active, setActive ] = useState('Billing');

    const nav = useNavigate()

    const topLinks = VIEWS.filter((view) => view.section == Section.Top).map((view) => (
        <a className={cx(classes.link, { [classes.linkActive]: view.name === active })}
            key={view.name} href={view.path}
            onClick={(event) => {
                event.preventDefault();
                setActive(view.name);
                nav(view.path)
            }}
        >
            <view.icon className={classes.linkIcon} stroke={1.5} />
            <span>{view.name}</span>
        </a>
    ));

    const bottomLinks = VIEWS.filter((view) => view.section == Section.Bottom)


    return (
        <Navbar width={{ base:300, sm: 300 }} p="xs">
            <Navbar.Section grow>
                {topLinks}
            </Navbar.Section>

            <Navbar.Section className={classes.footer}>
                {
                    bottomLinks.map((view) =>
                        <a href="#" className={classes.link} onClick={(event) => {
                            event.preventDefault();
                            nav(view.path)
                        }}>
                            { view.icon({className: classes.linkIcon, stroke: 1.5}) }
                            <span>{view.name}</span>
                        </a>
                    )
                }
            </Navbar.Section>
        </Navbar>
    );
}

