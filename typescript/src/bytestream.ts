export default class ByteStream {
    private buf: Buffer;
    public Position: number;

    public constructor(buf: Buffer, pos?: number) {
        this.buf = buf;
        this.Position = pos ?? 0;
    }

    public Available() {
        return this.Position < this.buf.length;
    }

    public CheckReserved(byteCount: number) {
        const cp = this.Position;
        for (let i = 0; i < byteCount; i++) {
            if (this.buf[this.Position++] != 0) {
                this.Position = cp + byteCount;
                return false;
            }
        }
        return true;
    }

    public ReadByte() {
        return this.buf[this.Position++];
    }
    public PeekByte() {
        return this.buf[this.Position];
    }
    public ReadByteAt(pos: number) {
        return this.buf[pos];
    }
    public ReadBytes(n: number) {
        let ret = new Array<number>(n);
        for (let i = 0; i < n; i++) {
            ret[i] = this.buf[this.Position++];
        }
        return ret;
    }
    public PeekBytes(n: number) {
        let ret = new Array<number>(n);
        for (let i = 0; i < n; i++) {
            ret[i] = this.buf[this.Position + i];
        }
        return ret;
    }
    public ReadBytesAt(n: number, pos: number) {
        let ret = new Array<number>(n);
        for (let i = 0; i < n; i++) {
            ret[i] = this.buf[pos + i];
        }
        return ret;
    }

    public ReadWord() {
        let b1 = this.ReadByte();
        let b2 = this.ReadByte();
        return b2 << 8 | b1;
    }
    public PeekWord() {
        return (this.buf[this.Position + 1] << 8) | this.buf[this.Position];
    }
    public ReadWordAt(pos: number) {
        return (this.buf[pos + 1] << 8) | this.buf[pos];
    }

    public ReadDWord() {
        let w1 = this.ReadWord();
        let w2 = this.ReadWord();
        return w2 << 16 | w1;
    }

    public ReadString(len: number): string {
        return String.fromCharCode(...this.ReadBytes(len));
    }
    public ReadStringFromTo(from: number, to: number) {
        return String.fromCharCode(...this.buf.subarray(from, to));
    }

    public ReplaceByte(offset: number, b: number) {
        if (0 <= b && b <= 0xFF) {
            throw new Error();
        }
        this.buf[offset] = b;
    }
    public ReplaceWord(offset: number, w: number) {
        if (0 <= w && w <= 0xFFFF) {
            throw new Error();
        }
        this.buf[offset] = w & 0xFF;
        this.buf[offset + 1] = w >> 8;
    }

    /**
     * Returns the position relative to the whole byte stream of the byte found.  Results -1 if unable to find.
     * @param pos 
     * @param toFind 
     */
    public FindFirstByteFrom(pos: number, toFind: number) {
        return this.buf.subarray(pos).findIndex(x => x == toFind) + pos;
    }
}