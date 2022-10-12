// Local Imports
import { View } from "../../Views";
import { FILES } from "../../../modules/File";
import {CRC32} from "../../../modules/Hash";

// External Imports
import { IconFile } from "@tabler/icons";
import {useParams} from "react-router-dom";


const NAME = "File"
const BASE_PATH = "/eter/file/"
const PATH = BASE_PATH + ":hash"


export default class implements View {
    name = NAME
    path = PATH
    description = "File details"
    icon = IconFile
    section = null
    component = component
}


function component(): JSX.Element {
    let { hash } = useParams()
    let hashIndex: CRC32 = new CRC32(parseInt(hash))

    const file = FILES.filter((file) => file.hash == hashIndex)
    
    return (
        <>
            <table>
                <th>Test</th>
                <td>1</td>
            </table>

            <h2>{file.name}</h2>
        </>
    )
}


export function hrefPath(hash: CRC32) {
    return BASE_PATH + hash.hash
}

