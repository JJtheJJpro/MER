import ByteStream from "../bytestream";

export enum ResourceTypes {
    Cursor = 0x8001,
    Bitmap,
    Icon,
    Menu,
    Dialog,
    String,
    FontDir,
    Font,
    Accelerator,
    RCData,
    MessageTable,
    GroupCursor,
    GroupIcon = 0x800E,
    Version = 0x8010,
    DLGInclude,
    PlugPlay = 0x8013,
    VXD,
    AniCursor,
    AniIcon,
    HTML,
    Manifest,
}

export class ResourceTable {
    AlignShift: number;
    Types: TypeInfo[];
    ResourceStrings: string[];

    private constructor(alignShift: number, types: TypeInfo[], resStrs: string[]) {
        this.AlignShift = alignShift;
        this.Types = types;
        this.ResourceStrings = resStrs;
    }

    public static Read(bst: ByteStream, length: number) {
        const _bstStart = bst.Position;

        const alignShift = bst.ReadWord();

        const types: TypeInfo[] = [];
        while (bst.PeekWord() != 0) {
            types.push(TypeInfo.Read(bst));
        }
        bst.ReadWord(); // garbage

        // alright, this is weird.  The "if" check is literally just to make sure that the resource table doesn't actually have any strings.
        // It shows in every valid doc I've seen that it should exist, but the exe's I've been using to test this, it doesn't exist.
        if (bst.Position - _bstStart < length) {
            let resStrs: string[] = [];
            while (bst.PeekByte() != 0) {
                resStrs.push(bst.ReadString(bst.ReadByte()));
            }
            bst.ReadByte(); // also garbage
        }

        return new ResourceTable(alignShift, types, []);
    }
}

export class TypeInfo {
    TypeIDOrOffset: number;
    ResourceCount: number;
    NameInfo: NameInfo[];

    private constructor(typeId: number, resCount: number, nameInfo: NameInfo[]) {
        this.TypeIDOrOffset = typeId;
        this.ResourceCount = resCount;
        this.NameInfo = nameInfo;
    }

    public static Read(bst: ByteStream) {
        const typeid = bst.ReadWord();
        const nRes = bst.ReadWord();
        if (!bst.CheckReserved(4)) {
            debugger;
        }

        const names: NameInfo[] = [];
        for (let i = 0; i < nRes; i++) {
            names.push(NameInfo.Read(bst));
        }

        return new TypeInfo(typeid, nRes, names);
    }
}

export class NameInfo {
    Offset: number;
    Length: number;
    Flags: number;
    ID: number;

    private constructor(offset: number, len: number, flags: number, id: number) {
        this.Offset = offset;
        this.Length = len;
        this.Flags = flags;
        this.ID = id;
    }

    public static Read(bst: ByteStream) {
        const offset = bst.ReadWord();
        const length = bst.ReadWord();
        const flag = bst.ReadWord();
        const resid = bst.ReadWord();
        if (!bst.CheckReserved(4)) {
            debugger;
        }

        return new NameInfo(offset, length, flag, resid);
    }
}