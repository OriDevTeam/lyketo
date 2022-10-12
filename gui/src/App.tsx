// Local Imports
import Navbar from "./app/Navbar";
import Header from "./app/Header";
import Views, {VIEWS} from "./app/Views";
import "./assets/App.css";


// External Imports
import { AppShell } from "@mantine/core";


export const VERSION = "0.1.0dev"


export default function(): JSX.Element {

  return (
      <AppShell navbar={Navbar()} header={Header()}
                styles={(theme) => ({
                    main: { backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0] },
                })}
      >
          { Views(VIEWS) }
      </AppShell>
  )
}

