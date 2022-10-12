// Local Imports
import {Section, View} from "../Views";

// External Imports
import { IconHelp } from "@tabler/icons";


export const NAME: string = "About"
export const PATH: string = "/about"


export default class implements View {
    name = NAME
    path = PATH
    section = Section.Bottom
    description = ""
    icon = IconHelp
    component = component
}


function component (): JSX.Element {
    return (
        <>
            About
        </>
    )
}

