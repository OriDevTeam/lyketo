// Local Imports

// External Imports
import {useState} from "react";
import { BrowserRouter } from "react-router-dom";
import {CookiesProvider} from "react-cookie";
import {useColorScheme} from "@mantine/hooks";
import {MantineProvider, ColorSchemeProvider, Global, ColorScheme} from '@mantine/core';


export default function ({ children }): JSX.Element {
    const preferredColorScheme = useColorScheme();
    const [colorScheme, setColorScheme] = useState<ColorScheme>(preferredColorScheme);
    const toggleColorScheme = (value?: ColorScheme) =>
        setColorScheme(value || (colorScheme === 'dark' ? 'light' : 'dark'));

    const theme = {
        loader: 'oval',
        fontFamily: 'Open Sans, sans serif',
        components: {
            Checkbox: { styles: { input: { cursor: 'pointer' }, label: { cursor: 'pointer' }}},
            TextInput: { styles: { label: { marginTop: '0.5rem' }}},
            Select: { styles: { label: { marginTop: '0.5rem' }}},
            Loader: { defaultProps: { size: 'xl' }},
            Space: { defaultProps: {h: 'sm' }},
            Anchor: { defaultProps: {target: '_blank' }}
        }
    }

    const styles = () => ({
        '.row': {
            display: 'flex',
            alignItems: 'flex-end',
            '& > div': {
                flexGrow: 1,
            }
        },
        '.rowCenter': {
            display: 'flex',
            alignItems: 'center',
            '& > div': {
                flexGrow: 1,
            }
        },
        '.embeddedInput': {
            display: 'inline-block',
            margin: 'auto 5px',
        }
    });

    return (
        <MantineProvider withGlobalStyles withNormalizeCSS withCSSVariables>
            <ColorSchemeProvider colorScheme={colorScheme} toggleColorScheme={toggleColorScheme}>
                <BrowserRouter>
                    <CookiesProvider>
                        <Global styles={styles} />
                        { children }
                    </CookiesProvider>
                </BrowserRouter>
            </ColorSchemeProvider>
        </MantineProvider>
    );
}
