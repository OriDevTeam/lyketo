// Local Imports
import {IconHome2, TablerIcon} from "@tabler/icons";
import {Section, View} from "../Views";

// External Imports


export default class implements View {
    name = "Landing"
    path = "/"
    description = "Landing"
    icon = IconHome2
    section = Section.Top
    component = component

}


function component(): JSX.Element {
    return (
        <>
            Landing
        </>
    )
}
