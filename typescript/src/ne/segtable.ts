import ByteStream from "../bytestream";

export class SegmentTable {
    SegmentDataOffset: number;
    SegmentLength: number;
    Flags: number;
    MinimumSegmentAllocationSize: number;

    public constructor(segDataOff: number, segLen: number, flags: number, minSegAllocSize: number) {
        this.SegmentDataOffset = segDataOff;
        this.SegmentLength = segLen;
        this.Flags = flags;
        this.MinimumSegmentAllocationSize = minSegAllocSize;
    }

    public static Read(bst: ByteStream) {
        return new SegmentTable(bst.ReadWord(), bst.ReadWord(), bst.ReadWord(), bst.ReadWord());
    }
}