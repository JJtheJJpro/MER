import ByteStream from "../bytestream";
import { ResourceTable } from "./restable";
import { SegmentTable } from "./segtable";

/**
 * An Executable extension added to a DOS program, provided by Microsoft Windows, used only in 16-bit environments
 */
export class NewExecutable {
    Signature: "NE";
    LinkerVersion: string;
    EntryTableOffset: number;
    EntryTableLength: number;
    CRC: number;
    FlagWord: number;
    AutomaticDataSegmentNumber: number;
    InitialLocalHeapSize: number;
    InitialStackSize: number;
    CS: number;
    IP: number;
    SS: number;
    SP: number;
    EntriesInSegmentTable: number;
    EntiresInModuleReferenceTable: number;
    NonResidentNamesTableSize: number;
    SegmentTableOffset: number;
    ResourceTableOffset: number;
    ResidentNamesTableOffset: number;
    ModuleReferenceTableOffset: number;
    ImportedNamesTableOffset: number;
    NonResidentNamesTableOffset: number;
    MoveableEntryPoints: number;
    ShiftCount: number;
    ResourceSegmentCount: number;
    TargetOS: number;
    AdditionalInfo: number;
    FastLoadOffset: number;
    FastLoadLength: number;
    ExpectedWindowsVersion: string;



    private constructor(magic: "NE",
        linkerVer: string,
        entryTblOff: number,
        entryTblLen: number,
        crc: number,
        flag: number,
        autoDataSegNum: number,
        initHeap: number,
        initStack: number,
        cs: number,
        ip: number,
        ss: number,
        sp: number,
        segTblEntries: number,
        modRefTblEntries: number,
        nonResNamesTblSize: number,
        segTblOff: number,
        resTblOff: number,
        resNameTblOff: number,
        modRefTblOff: number,
        impNamesTblOff: number,
        nonResNamesTblOff: number,
        moveEntryPts: number,
        shiftCount: number,
        resSegCount: number,
        targetOS: number,
        addInfo: number,
        fastLoadOff: number,
        fastLoadLen: number,
        expWinVer: string
    ) {
        this.Signature = magic;
        this.LinkerVersion = linkerVer;
        this.EntryTableOffset = entryTblOff;
        this.EntryTableLength = entryTblLen;
        this.CRC = crc;
        this.FlagWord = flag;
        this.AutomaticDataSegmentNumber = autoDataSegNum;
        this.InitialLocalHeapSize = initHeap;
        this.InitialStackSize = initStack;
        this.CS = cs;
        this.IP = ip;
        this.SS = ss;
        this.SP = sp;
        this.EntriesInSegmentTable = segTblEntries;
        this.EntiresInModuleReferenceTable = modRefTblEntries;
        this.NonResidentNamesTableSize = nonResNamesTblSize;
        this.SegmentTableOffset = segTblOff;
        this.ResourceTableOffset = resTblOff;
        this.ResidentNamesTableOffset = resNameTblOff;
        this.ModuleReferenceTableOffset = modRefTblOff;
        this.ImportedNamesTableOffset = impNamesTblOff;
        this.NonResidentNamesTableOffset = nonResNamesTblOff;
        this.MoveableEntryPoints = moveEntryPts;
        this.ShiftCount = shiftCount;
        this.ResourceSegmentCount = resSegCount;
        this.TargetOS = targetOS;
        this.AdditionalInfo = addInfo;
        this.FastLoadOffset = fastLoadOff;
        this.FastLoadLength = fastLoadLen;
        this.ExpectedWindowsVersion = expWinVer;
    }

    public static Read(bst: ByteStream, offset: number) {
        // magic (NE) managed by Executable class in reader.ts
        const linkerVer = `${bst.ReadByte()}.${bst.ReadByte()}`;
        const entryTblOff = bst.ReadWord();
        const entryTblLen = bst.ReadWord();
        const crc = bst.ReadDWord();
        const flag = bst.ReadWord();
        const autoDataSegNum = bst.ReadWord();
        const initHeap = bst.ReadWord();
        const initStack = bst.ReadWord();
        const ip = bst.ReadWord();
        const cs = bst.ReadWord();
        const sp = bst.ReadWord();
        const ss = bst.ReadWord();
        const segTblEntries = bst.ReadWord();
        const modRefTblEntries = bst.ReadWord();
        const nonResNamesTblSize = bst.ReadWord();
        const segTblOff = bst.ReadWord();
        const resTblOff = bst.ReadWord();
        const resNameTblOff = bst.ReadWord();
        const modRefTblOff = bst.ReadWord();
        const impNamesTblOff = bst.ReadWord();
        const nonResNamesTblOff = bst.ReadDWord() - offset;
        const moveEntryPts = bst.ReadWord();
        const shiftCount = bst.ReadWord();
        const resSegCount = bst.ReadWord();
        const targetOS = bst.ReadByte();

        const addInfo = bst.ReadByte();
        const fastLoadOff = bst.ReadWord();
        const fastLoadLen = bst.ReadWord();
        if (!bst.CheckReserved(2)) {
            throw new Error();
        }
        const expWinVerMin = bst.ReadByte();
        const expWinVerMaj = bst.ReadByte();

        if (bst.Position < segTblOff + offset) {
            let rawData = [];
            while (bst.Position < segTblOff) {
                rawData.push(bst.ReadByte());
            }
            debugger;
            // let it go on for now
        } else if (bst.Position > segTblOff + offset) {
            debugger;
            throw new Error();
        }
        const segments: SegmentTable[] = [];
        for (let i = 0; i < segTblEntries; i++) {
            segments.push(SegmentTable.Read(bst));
        }

        if (bst.Position < resTblOff + offset) {
            let rawData = [];
            while (bst.Position < segTblOff) {
                rawData.push(bst.ReadByte());
            }
            debugger;
            // let it go on for now
        } else if (bst.Position > resTblOff + offset) {
            debugger;
            throw new Error();
        }
        const resources = ResourceTable.Read(bst, resNameTblOff - resTblOff);

        return new NewExecutable("NE",
            linkerVer,
            entryTblOff,
            entryTblLen,
            crc,
            flag,
            autoDataSegNum,
            initHeap,
            initStack,
            cs, ip, ss, sp,
            segTblEntries,
            modRefTblEntries,
            nonResNamesTblSize,
            segTblOff,
            resTblOff,
            resNameTblOff,
            modRefTblOff,
            impNamesTblOff,
            nonResNamesTblOff,
            moveEntryPts,
            shiftCount,
            resSegCount,
            targetOS,
            addInfo,
            fastLoadOff,
            fastLoadLen,
            `${expWinVerMaj}.${expWinVerMin}`
        );
    }
}