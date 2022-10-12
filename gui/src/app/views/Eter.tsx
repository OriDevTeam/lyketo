// Local Imports
import {Section, View} from "../Views";
import DiskBrowser from "./eter/DiskBrowser";
import VirtualBrowser from "./eter/VirtualBrowser";
import Types from "./eter/Types";

// External Imports
import { Tabs } from "@mantine/core";
import { IconKey, TablerIcon} from "@tabler/icons";


export const Name = "Eter"
export const Path = "/eter"


export default class implements View {
    name = Name
    path = Path
    section = Section.Top
    description = ""
    icon = IconKey
    component = component
}


export interface Tab {
    name: string
    color: string
    icon: TablerIcon
    component: any
}


const TABS: Tab[] = [
    new DiskBrowser,
    new VirtualBrowser,
    new Types
]


export interface Type {
    value: string
    label: string
    color: string
}

export const TYPES: Type[] = [
    { value: 'encrypted_object0', color: "gray", label: 'Encrypted Object (Type 0)' },
    { value: 'encrypted_object1', color: "red", label: 'Encrypted Object (Type 1)' },
    { value: 'encrypted_object2', color: "brown", label: 'Encrypted Object (Type 2)' },
    { value: 'panama', color: "#094C75", label: 'Panama (Type 3)' },
    { value: 'hybrid_crypt', color: "teal", label: 'Hybrid Crypt (Type 4)' },
    { value: 'hybrid_crypt_plus', color: "green", label: 'Hybrid Crypt (Type 5)' },
    { value: 'encrypted_object_6', color: "orange", label: 'Encrypted Object (Type 6)' },
]



function component(): JSX.Element {

    
    return (
        <Tabs color={TABS[0].color} defaultValue={TABS[0].name}>
            <Tabs.List>
                {
                    TABS.map((tab) =>
                            <Tabs.Tab value={tab.name} color={tab.color} icon={tab.icon({size: 14})}>
                                {tab.name}
                            </Tabs.Tab>
                    )
                }
            </Tabs.List>

            {
                TABS.map((tab) =>
                        <Tabs.Panel value={tab.name} pt="xs">
                            { tab.component() }
                        </Tabs.Panel>
                )
            }
        </Tabs>
    )
}

