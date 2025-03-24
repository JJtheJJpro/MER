import x86_16 from "./byte-operation/x86_16";
import ByteStream from "./bytestream";

/**
 * Named after Mark Zbikowski, MZ is the main MS-DOS EXE format found in all executables and libraries, containing basic information of the whole executable.
 */
export class MZ {
    /**
     * The Signature, or "Magic" number that always starts the MZ header.
     */
    Signature: "MZ";
    /**
     * Number of bytes in the last page.
     */
    LastPageBytes: number;
    /**
     * Number of whole/partial pages.
     */
    PageCount: number;
    /**
     * Number of entries in the relocation table
     */
    RelocationTableEntryCount: number;
    /**
     * The number of paragraphs taken up by the header. It can be any value, as the loader just uses it to find where the actual executable data starts.
     * 
     * It may be larger than what the "standard" fields take up, and you may use it if you want to include your own header metadata, or put the relocation table there, or use it for any other purpose.
     */
    HeaderSize: number;
    /**
     * The number of paragraphs **required** by the program, excluding the PSP and program image. If no free block is big enough, the loading stops.
     */
    MinimumAllocation: number;
    /**
     * The number of paragraphs **requested** by the program. If no free block is big enough, the biggest one possible is allocated.
     */
    MaximumAllocation: number;
    /**
     * Relocatable segment address for SS.
     */
    InitialSS: number;
    /**
     * Initial value for SP.
     */
    InitialSP: number;
    /**
     * When added to the sum of all other words in the file, the result should be zero.
     */
    Checksum: number;
    /**
     * Initial value for IP.
     */
    InitialIP: number;
    /**
     * Relocatable segment address for CS.
     */
    InitialCS: number;
    /**
     * The (absolute) offset to the relocation table.
     */
    RelocationTableOffset: number;
    /**
     * Value used for overlay management. If zero, this is the main executable.
     */
    Overlay: number;

    /**
     * The OEM Id.  This value is only read in Windows executables.
     */
    OEMIdentifier?: number;
    /**
     * The OEM Info.  This value is only read in Windows executables.
     */
    OEMInfo?: number;
    /**
     * The starting address of the new header.
     */
    NewHeaderStart?: number;
    /**
     * The relocation tables.
     */
    RelocationTables: RelocationTable[];

    /**
     * The assembly code.
     */
    HeaderCode: string[];

    private constructor(magic: "MZ",
        lastPageBytes: number,
        pageCount: number,
        numEntriesRelocTable: number,
        headerSize: number,
        minAlloc: number,
        maxAlloc: number,
        initSS: number,
        initSP: number,
        checksum: number,
        initIP: number,
        initCS: number,
        relocTableOffset: number,
        overlay: number,
        headerCode: string[],
        relocTbls: RelocationTable[],
        oemid?: number,
        oeminfo?: number,
        newHeaderStart?: number
    ) {
        this.Signature = magic;
        this.LastPageBytes = lastPageBytes;
        this.PageCount = pageCount;
        this.RelocationTableEntryCount = numEntriesRelocTable;
        this.HeaderSize = headerSize;
        this.MinimumAllocation = minAlloc;
        this.MaximumAllocation = maxAlloc;
        this.InitialSS = initSS;
        this.InitialSP = initSP;
        this.Checksum = checksum;
        this.InitialIP = initIP;
        this.InitialCS = initCS;
        this.RelocationTableOffset = relocTableOffset;
        this.Overlay = overlay;
        this.OEMIdentifier = oemid;
        this.OEMInfo = oeminfo;
        this.NewHeaderStart = newHeaderStart;
        this.HeaderCode = headerCode;
        this.RelocationTables = relocTbls;
    }

    /**
     * Reads a byte stream.  Throws an error is the first two bytes don't match "MZ"
     * @param bst The byte stream
     * @returns An instance of the MZ class
     */
    public static Read(bst: ByteStream) {
        if (bst.ReadString(2) != "MZ") {
            throw new Error();
        }

        const lastPageBytes = bst.ReadWord();
        const pageCount = bst.ReadWord();
        const numEntriesRelocTable = bst.ReadWord();
        const headerSize = bst.ReadWord();
        const minAlloc = bst.ReadWord();
        const maxAlloc = bst.ReadWord();
        const initSS = bst.ReadWord();
        const initSP = bst.ReadWord();
        const checksum = bst.ReadWord();
        const initIP = bst.ReadWord();
        const initCS = bst.ReadWord();
        const relocTableOffset = bst.ReadWord();
        const overlay = bst.ReadWord();

        let oemid: number | undefined = undefined;
        let oeminfo: number | undefined = undefined;
        let newHeaderStart: number | undefined = undefined;
        bst.Position += 8; // skip reserved
        oemid = bst.ReadWord();
        oeminfo = bst.ReadWord();
        bst.Position += 20; // skip reserved
        newHeaderStart = bst.ReadDWord();

        let relocTbls: RelocationTable[] = [];
        if (bst.Position == relocTableOffset && numEntriesRelocTable > 0) {
            for (let i = 0; i < numEntriesRelocTable; i++) {
                relocTbls.push(RelocationTable.Read(bst));
            }
        }

        if (bst.Position < headerSize * 16) {
            if (!bst.CheckReserved((headerSize * 16) - bst.Position)) {
                debugger;
            }
        }

        if (!newHeaderStart) {
            throw new Error();
        }

        x86_16.ss = initSS;
        x86_16.sp = initSP;
        x86_16.ip = initIP;
        x86_16.cs = initCS;
        let bytes = bst.ReadBytes(newHeaderStart - headerSize * 16);
        let code = x86_16.parseCode(bytes);

        return new MZ("MZ", lastPageBytes, pageCount, numEntriesRelocTable, headerSize, minAlloc, maxAlloc, initSS, initSP, checksum, initIP, initCS, relocTableOffset, overlay, code, relocTbls, oemid, oeminfo, newHeaderStart);
    }
}

export class RelocationTable {
    Offset: number;
    Segment: number;

    private constructor(offset: number, segment: number) {
        this.Offset = offset;
        this.Segment = segment;
    }

    public static Read(bst: ByteStream) {
        return new RelocationTable(bst.ReadWord(), bst.ReadWord());
    }
}