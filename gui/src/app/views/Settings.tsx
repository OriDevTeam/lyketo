// Local Imports
import {Section, View} from "../Views";

// External Imports
import {IconHelp, IconSettings} from "@tabler/icons";


export const NAME = "Settings"
export const PATH = "/settings"


export default class implements View {
    name = NAME
    path = PATH
    section = Section.Bottom
    description = ""
    icon = IconSettings
    component = component
}


function component (): JSX.Element {
    return (
        <>
            Settings
        </>
    )
}

