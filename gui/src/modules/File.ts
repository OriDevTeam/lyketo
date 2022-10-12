// Local Imports
import {Type} from "./Eter";
import {CRC32} from "./Hash";


// External Imports


export interface File {
    name: string
    hash: CRC32 // Index for file browser
    size: number
    created: number
    modified: number
    type: number
}


export const FILES: File[] = [
    { name: "EncObjT0.epk", hash: new CRC32(1), size: 1500000, created: 0, modified: 0, type: 0 },
    { name: "EncObjT0.eix", hash: new CRC32(2), size: 2000, created: 0, modified: 0, type: 0 },


    { name: "EncObjT1.epk", hash: new CRC32(3), size: 1300000, created: 0, modified: 0, type: 1},
    { name: "EncObjT1.eix", hash: new CRC32(4), size: 2000, created: 0, modified: 0, type: 1},

    { name: "EncObjT2.epk", hash: new CRC32(5), size: 1000000, created: 0, modified: 0, type: 2},
    { name: "EncObjT2.eix", hash: new CRC32(6), size: 2000, created: 0, modified: 0, type: 2},

    { name: "EncObjT6.epk", hash: new CRC32(7), size: 1000000, created: 0, modified: 0, type: 6},
    { name: "EncObjT6.eix", hash: new CRC32(8), size: 2000, created: 0, modified: 0, type: 6},

    { name: "Panama", hash: new CRC32(9), size: 400000, created: 0, modified: 0, type: 3},

    { name: "HybCry", hash: new CRC32(10), size: 400000, created: 0, modified: 0, type: 4},

    { name: "HybCryPlus", hash: new CRC32(11), size: 780000, created: 0, modified: 0, type: 5},
    { name: "HybCryPlusA", hash: new CRC32(12), size: 780000, created: 0, modified: 0, type: 5},
    { name: "HybCryPlusB", hash: new CRC32(13), size: 780000, created: 0, modified: 0, type: 5},
    { name: "HybCryPlusC", hash: new CRC32(14), size: 780000, created: 0, modified: 0, type: 5},

]


