// Local Imports
import Eter from "./views/Eter";
import { TablerIcon } from "@tabler/icons";
import Settings from "./views/Settings";
import About from "./views/About";
import Landing from "./views/Landing";

// External Imports
import {Navigate, Route, Routes} from "react-router-dom";
import { Suspense } from "react";


export enum Section {
    Top,
    Bottom
}

export interface View {
    name: string
    path: string
    description: string
    icon: TablerIcon
    section : Section | null
    component: any
}


export const VIEWS: View[] = [
    new Landing,
    new Eter,
    new Settings,
    new About
]


export default function (views: View[]): JSX.Element {

    return (
        <Routes>
            {
                views.map((view, index) =>
                    <Route key={index} path={view.path} element={
                        <Suspense>
                            <view.component />
                        </Suspense>
                    }/>
                )
            }
        </Routes>
    )
}


