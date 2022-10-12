// Local Imports
import {Tab} from "../../Eter";
import {IconOld, IconWebhook} from "@tabler/icons";

// External Imports


export default class implements Tab {
    name = "Encrypted Object"
    color = "green"
    icon = IconWebhook
    component = component
}


export function component () {
    return (
        <>
            <h3>Encrypted Object</h3>
        </>
    )
}

