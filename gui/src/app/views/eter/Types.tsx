// Local Imports
import { Tab } from "../Eter";
import Panama from "./types/Panama";
import HybridCrypt from "./types/HybridCrypt";
import EncryptedObject from "./types/EncryptedObject";

// External Imports
import { Tabs } from "@mantine/core";
import { IconFileTypography } from "@tabler/icons";


export default class implements Tab {
    name = "Types"
    color = "orange"
    icon = IconFileTypography
    component = component
}


const TABS: Tab[] = [
    new EncryptedObject,
    new Panama,
    new HybridCrypt
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

