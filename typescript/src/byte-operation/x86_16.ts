import ByteStream from "../bytestream";
import { x86_16_HiRegType, x86_16_LoRegType, x86_16_RegType } from "./types";

const ops: ["add", "or", "adc", "sbb", "and", "sub", "xor", "cmp"] = ["add", "or", "adc", "sbb", "and", "sub", "xor", "cmp"];
const regNames: ['ax', 'cx', 'dx', 'bx', 'sp', 'bp', 'si', 'di'] = ['ax', 'cx', 'dx', 'bx', 'sp', 'bp', 'si', 'di'];
const rmNames: ['bx+si', 'bx+di', 'bp+si', 'bp+di', 'si', 'di', 'bp', 'bx'] = ['bx+si', 'bx+di', 'bp+si', 'bp+di', 'si', 'di', 'bp', 'bx'];
const segRegNames: ['es', 'cs', 'ss', 'ds'] = ['es', 'cs', 'ss', 'ds'];

/**
 * This class represents the registry list of the x86 instruction set for 16-bit code in the form of TypeScript.
 */
export default class x86_16 {
    static stack: (number | x86_16_RegType)[] = [];

    static ah: number | x86_16_HiRegType = 'ah';
    static al: number | x86_16_LoRegType = 'al';
    static bh: number | x86_16_HiRegType = 'bh';
    static bl: number | x86_16_LoRegType = 'bl';
    static ch: number | x86_16_HiRegType = 'ch';
    static cl: number | x86_16_LoRegType = 'cl';
    static dh: number | x86_16_HiRegType = 'dh';
    static dl: number | x86_16_LoRegType = 'dl';

    static si: number | x86_16_RegType = 'si';
    static di: number | x86_16_RegType = 'di';
    static bp: number | x86_16_RegType = 'bp';
    static sp: number | x86_16_RegType = 'sp';

    static cs: number | x86_16_RegType = 'cs';
    static ds: number | x86_16_RegType = 'ds';
    static ss: number | x86_16_RegType = 'ss';
    static es: number | x86_16_RegType = 'es';

    static ip: number | x86_16_RegType = 'ip';

    static cf: 0 | 1 = 0;
    static pf: 0 | 1 = 0;
    static af: 0 | 1 = 0;
    static zf: 0 | 1 = 0;
    static sf: 0 | 1 = 0;
    static tf: 0 | 1 = 0;
    static if: 0 | 1 = 0;
    static df: 0 | 1 = 0;
    static of: 0 | 1 = 0;
    static iopl: 0 | 1 | 2 | 3 = 0;
    static nt: 0 | 1 = 0;
    static get flags(): number {
        return (x86_16.nt << 14) | (x86_16.iopl << 12) | (x86_16.of << 11) | (x86_16.df << 10) | (x86_16.if << 9) | (x86_16.tf << 8) | (x86_16.sf << 7) | (x86_16.zf << 6) | (x86_16.af << 4) | (x86_16.pf << 2) | x86_16.cf;
    }
    static set flags(value: number) {
        x86_16.cf = (value & 0b1) as (0 | 1);
        x86_16.pf = ((value >> 2) & 0b1) as (0 | 1);
        x86_16.af = ((value >> 4) & 0b1) as (0 | 1);
        x86_16.zf = ((value >> 6) & 0b1) as (0 | 1);
        x86_16.sf = ((value >> 7) & 0b1) as (0 | 1);
        x86_16.tf = ((value >> 8) & 0b1) as (0 | 1);
        x86_16.if = ((value >> 9) & 0b1) as (0 | 1);
        x86_16.df = ((value >> 10) & 0b1) as (0 | 1);
        x86_16.of = ((value >> 11) & 0b1) as (0 | 1);
        x86_16.iopl = ((value >> 12) & 0b11) as (0 | 1 | 2 | 3);
        x86_16.nt = ((value >> 14) & 0b1) as (0 | 1);
    }

    private static rt_ax: x86_16_RegType = 'ax';
    static get ax(): number | x86_16_RegType {
        if (typeof this.ah === 'string' || typeof this.al == 'string') {
            //debugger;
            if (typeof this.ah === 'string' && typeof this.al == 'string') {
                return x86_16.rt_ax;
            }
            console.error("WARNING: REGISTRY AX IS PARTIALLY DEFINED, REPLACING UNKNOWN VALUES WITH 0.");
        }
        return (typeof this.ah === 'string' ? 0 : this.ah << 8) | (typeof this.al === 'string' ? 0 : this.al);
    }
    static set ax(value: number | x86_16_RegType) {
        if (typeof value == 'number') {
            this.ah = value >> 8;
            this.al = value & 0xFF;
        } else {
            x86_16.rt_ax = value;
        }
    }

    private static rt_bx: x86_16_RegType = 'bx';
    static get bx(): number | x86_16_RegType {
        if (typeof this.bh === 'string' || typeof this.bl == 'string') {
            //debugger;
            if (typeof this.bh === 'string' && typeof this.bl == 'string') {
                return x86_16.rt_bx;
            }
            console.error("WARNING: REGISTRY BX IS PARTIALLY DEFINED, REPLACING UNKNOWN VALUES WITH 0.");
        }
        return (typeof this.bh === 'string' ? 0 : this.bh << 8) | (typeof this.bl === 'string' ? 0 : this.bl);
    }
    static set bx(value: number | x86_16_RegType) {
        if (typeof value == 'number') {
            this.bh = value >> 8;
            this.bl = value & 0xFF;
        } else {
            x86_16.rt_bx = value;
        }
    }

    private static rt_cx: x86_16_RegType = 'cx';
    static get cx(): number | x86_16_RegType {
        if (typeof this.ch === 'string' || typeof this.cl == 'string') {
            //debugger;
            if (typeof this.ch === 'string' && typeof this.cl == 'string') {
                return x86_16.rt_cx;
            }
            console.error("WARNING: REGISTRY CX IS PARTIALLY DEFINED, REPLACING UNKNOWN VALUES WITH 0.");
        }
        return (typeof this.ch === 'string' ? 0 : this.ch << 8) | (typeof this.cl === 'string' ? 0 : this.cl);
    }
    static set cx(value: number | x86_16_RegType) {
        if (typeof value == 'number') {
            this.ch = value >> 8;
            this.cl = value & 0xFF;
        } else {
            x86_16.rt_cx = value;
        }
    }

    private static rt_dx: x86_16_RegType = 'dx';
    static get dx(): number | x86_16_RegType {
        if (typeof this.dh === 'string' || typeof this.dl == 'string') {
            //debugger;
            if (typeof this.dh === 'string' && typeof this.dl == 'string') {
                return x86_16.rt_dx;
            }
            console.error("WARNING: REGISTRY DX IS PARTIALLY DEFINED, REPLACING UNKNOWN VALUES WITH 0.");
        }
        return (typeof this.dh === 'string' ? 0 : this.dh << 8) | (typeof this.dl === 'string' ? 0 : this.dl);
    }
    static set dx(value: number | x86_16_RegType) {
        if (typeof value == 'number') {
            this.dh = value >> 8;
            this.dl = value & 0xFF;
        } else {
            x86_16.rt_dx = value;
        }
    }

    //#region Code Parsing

    // The following are helper functions for the opcode functions.

    private static modrmByteHandling(bst: ByteStream) {
        const modbyte = bst.ReadByte();
        const mod = modbyte >> 6;
        const reg = (modbyte >> 3) & 0b111;
        const rm = modbyte & 0b111;

        let displacement = 0;
        let v: number | undefined = undefined;
        let vS: string | undefined = undefined;

        if (mod == 0 || mod == 1 || mod == 2) {
            v = 0;
            switch (mod) {
                case 0:
                    if (rm == 6) {
                        v = displacement = bst.ReadWord();
                        vS = `[0x${displacement.toString(16).toUpperCase()}]`;
                    } else {
                        switch (rm) {
                            case 0:
                                if (typeof x86_16.bx == 'number' && typeof x86_16.si == 'number') {
                                    v = x86_16.bx + x86_16.si;
                                } else {
                                    throw new Error();
                                }
                                break;
                            case 1:
                                if (typeof x86_16.bx == 'number' && typeof x86_16.di == 'number') {
                                    v = x86_16.bx + x86_16.di;
                                } else {
                                    throw new Error();
                                }
                                break;
                            case 2:
                                if (typeof x86_16.bp == 'number' && typeof x86_16.si == 'number') {
                                    v = x86_16.bp + x86_16.si;
                                } else {
                                    throw new Error();
                                }
                                break;
                            case 3:
                                if (typeof x86_16.bp == 'number' && typeof x86_16.di == 'number') {
                                    v = x86_16.bp + x86_16.di;
                                } else {
                                    throw new Error();
                                }
                                break;
                            case 4:
                                if (typeof x86_16.si == 'number') {
                                    v = x86_16.si;
                                } else {
                                    throw new Error();
                                }
                                break;
                            case 5:
                                if (typeof x86_16.di == 'number') {
                                    v = x86_16.di;
                                } else {
                                    throw new Error();
                                }
                                break;
                            case 7:
                                if (typeof x86_16.bx == 'number') {
                                    v = x86_16.bx;
                                } else {
                                    throw new Error();
                                }
                                break;
                        }
                        vS = `[${rmNames[rm]}]`;
                    }
                    break;
                case 1:
                    displacement = bst.ReadByte();
                    switch (rm) {
                        case 0:
                            if (typeof x86_16.bx == 'number' && typeof x86_16.si == 'number') {
                                v = x86_16.bx + x86_16.si;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 1:
                            if (typeof x86_16.bx == 'number' && typeof x86_16.di == 'number') {
                                v = x86_16.bx + x86_16.di;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 2:
                            if (typeof x86_16.bp == 'number' && typeof x86_16.si == 'number') {
                                v = x86_16.bp + x86_16.si;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 3:
                            if (typeof x86_16.bp == 'number' && typeof x86_16.di == 'number') {
                                v = x86_16.bp + x86_16.di;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 4:
                            if (typeof x86_16.si == 'number') {
                                v = x86_16.si;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 5:
                            if (typeof x86_16.di == 'number') {
                                v = x86_16.di;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 6:
                            if (typeof x86_16.bp == 'number') {
                                v = x86_16.bp;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 7:
                            if (typeof x86_16.bx == 'number') {
                                v = x86_16.bx;
                            } else {
                                throw new Error();
                            }
                            break;
                    }
                    v += displacement;
                    vS = `[${rmNames[rm]}+0x${displacement?.toString(16).toUpperCase()}]`;
                    break;
                case 2:
                    displacement = bst.ReadWord();
                    switch (rm) {
                        case 0:
                            if (typeof x86_16.bx == 'number' && typeof x86_16.si == 'number') {
                                v = x86_16.bx + x86_16.si;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 1:
                            if (typeof x86_16.bx == 'number' && typeof x86_16.di == 'number') {
                                v = x86_16.bx + x86_16.di;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 2:
                            if (typeof x86_16.bp == 'number' && typeof x86_16.si == 'number') {
                                v = x86_16.bp + x86_16.si;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 3:
                            if (typeof x86_16.bp == 'number' && typeof x86_16.di == 'number') {
                                v = x86_16.bp + x86_16.di;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 4:
                            if (typeof x86_16.si == 'number') {
                                v = x86_16.si;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 5:
                            if (typeof x86_16.di == 'number') {
                                v = x86_16.di;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 6:
                            if (typeof x86_16.bp == 'number') {
                                v = x86_16.bp;
                            } else {
                                throw new Error();
                            }
                            break;
                        case 7:
                            if (typeof x86_16.bx == 'number') {
                                v = x86_16.bx;
                            } else {
                                throw new Error();
                            }
                            break;
                    }
                    vS = `[${rmNames[rm]}+0x${displacement?.toString(16).toUpperCase()}]`;
                    break;
            }
        }

        return {
            modbyte,
            mod,
            reg,
            rm,
            displacement,
            v,
            vS
        }
    }

    // The following are functions that handle each operation code and manipulate registry values, the stack, and the byte stream itself if needed and return the corresponding assembly code in string.

    static op00() {
        return "nop";
    }
    static op0E() {
        x86_16.stack.push(x86_16.cs);
        return "push cs";
    }
    static op1F() {
        x86_16.ds = x86_16.stack.pop() ?? (() => { throw new Error(); })();
        return "pop ds";
    }
    static op33(bst: ByteStream) {
        const { mod, reg, rm, v, vS } = this.modrmByteHandling(bst);
        const destreg = regNames[reg];

        if (v) {
            if (typeof x86_16.ds == 'number' && (rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod == 0) || rm == 7)) {
                const index = (x86_16.ds << 4) + v;
                if (typeof x86_16[destreg] == 'number') {
                    x86_16[destreg] ^= bst.ReadWordAt(index);
                } else {
                    throw new Error();
                }
            } else if (typeof x86_16.ss == 'number' && (rm == 2 || rm == 3 || (rm == 6 && mod != 0))) {
                const index = (x86_16.ss << 4) + v;
                if (typeof x86_16[destreg] == 'number') {
                    x86_16[destreg] ^= bst.ReadWordAt(index);
                } else {
                    throw new Error();
                }
            } else {
                throw new Error();
            }
        } else {
            const regName = regNames[rm];
            if (typeof x86_16[destreg] == 'number' && typeof x86_16[regName] == 'number') {
                x86_16[destreg] ^= x86_16[regName];
            } else {
                throw new Error();
            }
        }

        return `xor ${destreg},${vS ?? regNames[rm]}`;
    }
    static op50() {
        x86_16.stack.push(x86_16.ax);
        return "push ax";
    }
    static op55() {
        x86_16.stack.push(x86_16.bp);
        return "push bp";
    }
    static op56() {
        x86_16.stack.push(x86_16.si);
        return "push si";
    }
    static op5D() {
        x86_16.bp = x86_16.stack.pop() ?? (() => { throw new Error(); })();
        return "pop bp";
    }
    static op81(bst: ByteStream) {
        const { reg, rm, v, vS } = this.modrmByteHandling(bst);

        const immediate = bst.ReadWord();
        const mnemonic = ops[reg];

        switch (mnemonic) {
            case 'add':
                break;
            case 'or':
                break;
            case 'adc':
                break;
            case 'sbb':
                break;
            case 'and':
                break;
            case 'sub':
                if (v) {
                    bst.ReplaceWord(v, bst.ReadWordAt(v) - immediate);
                } else {
                    if (typeof x86_16[regNames[rm]] == 'number') {
                        (x86_16[regNames[rm]] as number) -= immediate;
                    } else {
                        throw new Error();
                    }
                }
                break;
            case 'xor':
                break;
            case 'cmp':
                break;
        }

        return `${mnemonic} ${vS ?? regNames[rm]},0x${immediate.toString(16).toUpperCase()}`;
    }
    static op83(bst: ByteStream) {
        const { reg, rm, v, vS } = this.modrmByteHandling(bst);

        let immediate = bst.ReadByte();
        if (immediate & 0x80) {
            immediate -= 0x100;
        }

        const mnemonic = ops[reg];

        switch (mnemonic) {
            case 'add':
                break;
            case 'or':
                break;
            case 'adc':
                break;
            case 'sbb':
                break;
            case 'and':
                break;
            case 'sub':
                if (v) {
                    bst.ReplaceByte(v, bst.ReadByteAt(v) - immediate);
                } else {
                    if (typeof x86_16[regNames[rm]] == 'number') {
                        x86_16[regNames[rm]] = x86_16[regNames[rm]] as number - immediate;
                    } else {
                        throw new Error();
                    }
                }
                break;
            case 'xor':
                break;
            case 'cmp':
                break;
        }

        return `${mnemonic} ${vS ?? regNames[rm]},0x${immediate.toString(16).toUpperCase()}`;
    }
    static op8B(bst: ByteStream) {
        const { mod, reg, rm, v, vS } = this.modrmByteHandling(bst);
        const destreg = regNames[reg];

        if (v) {
            if (typeof x86_16.ds == 'number' && (rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod == 0) || rm == 7)) {
                const index = (x86_16.ds << 4) + v;
                x86_16[destreg] = bst.ReadWordAt(index);
            } else if (typeof x86_16.ss == 'number' && (rm == 2 || rm == 3 || (rm == 6 && mod != 0))) {
                const index = (x86_16.ss << 4) + v;
                x86_16[destreg] = bst.ReadWordAt(index);
            } else {
                throw new Error();
            }
        } else {
            x86_16[destreg] = x86_16[regNames[rm]];
        }

        return `mov ${destreg},${vS ?? regNames[rm]}`;
    }
    static op8C(bst: ByteStream) {
        const { mod, reg, rm, v, vS } = this.modrmByteHandling(bst);

        const segReg = segRegNames[reg];

        if (v) {
            if (typeof x86_16.ds == 'number' && (rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod == 0) || rm == 7)) {
                const index = (x86_16.ds << 4) + v;
                if (typeof x86_16[segReg] == 'number') {
                    bst.ReplaceWord(index, x86_16[segReg]);
                } else {
                    throw new Error();
                }
            } else if (typeof x86_16.ss == 'number' && (rm == 2 || rm == 3 || (rm == 6 && mod != 0))) {
                const index = (x86_16.ss << 4) + v;
                if (typeof x86_16[segReg] == 'number') {
                    bst.ReplaceWord(index, x86_16[segReg]);
                } else {
                    throw new Error();
                }
            } else {
                throw new Error();
            }
        } else {
            x86_16[regNames[rm]] = x86_16[segReg];
        }

        return `mov ${vS ?? regNames[rm]},${segReg}`;
    }
    static op8D(bst: ByteStream) {
        const { reg, rm, v, vS } = this.modrmByteHandling(bst);
        x86_16[regNames[reg]] = v ?? x86_16[regNames[rm]];
        return `lea ${regNames[reg]},${vS ?? regNames[rm]}`;
    }
    static op8E(bst: ByteStream) {
        const { mod, reg, rm, v, vS } = this.modrmByteHandling(bst);

        const segReg = segRegNames[reg];

        if (v) {
            if (typeof x86_16.ds == 'number' && (rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod == 0) || rm == 7)) {
                const index = (x86_16.ds << 4) + v;
                x86_16[segReg] = bst.ReadWordAt(index);
            } else if (typeof x86_16.ss == 'number' && (rm == 2 || rm == 3 || (rm == 6 && mod != 0))) {
                const index = (x86_16.ss << 4) + v;
                x86_16[segReg] = bst.ReadWordAt(index);
            } else {
                throw new Error();
            }
        } else {
            x86_16[segReg] = x86_16[regNames[rm]];
        }

        return `mov ${segReg},${vS ?? regNames[rm]}`;
    }
    static opAE(bst: ByteStream) {
        if (typeof x86_16.es != 'number' || typeof x86_16.di != 'number') {
            throw new Error();
        }
        const ptrValue = bst.ReadByteAt((x86_16.es << 4) + x86_16.di);
        x86_16.zf = ptrValue == x86_16.al ? 1 : 0;
        x86_16.df ? x86_16.di-- : x86_16.di++;
        return "scasb";
    }
    static opB0(bst: ByteStream) {
        return `mov al,0x${(x86_16.al = bst.ReadByte()).toString(16).toUpperCase().padStart(2, "0")}`;
    }
    static opB1(bst: ByteStream) {
        return `mov cl,0x${(x86_16.cl = bst.ReadByte()).toString(16).toUpperCase().padStart(2, "0")}`;
    }
    static opB2(bst: ByteStream) {
        return `mov dl,0x${(x86_16.dl = bst.ReadByte()).toString(16).toUpperCase().padStart(2, "0")}`;
    }
    static opB3(bst: ByteStream) {
        return `mov bl,0x${(x86_16.bl = bst.ReadByte()).toString(16).toUpperCase().padStart(2, "0")}`;
    }
    static opB4(bst: ByteStream) {
        return `mov ah,0x${(x86_16.ah = bst.ReadByte()).toString(16).toUpperCase().padStart(2, "0")}`;
    }
    static opB5(bst: ByteStream) {
        return `mov ch,0x${(x86_16.ch = bst.ReadByte()).toString(16).toUpperCase().padStart(2, "0")}`;
    }
    static opB6(bst: ByteStream) {
        return `mov dh,0x${(x86_16.dh = bst.ReadByte()).toString(16).toUpperCase().padStart(2, "0")}`;
    }
    static opB7(bst: ByteStream) {
        return `mov bh,0x${(x86_16.bh = bst.ReadByte()).toString(16).toUpperCase().padStart(2, "0")}`;
    }
    static opB8(bst: ByteStream) {
        return `mov ax,0x${(x86_16.ax = bst.ReadWord()).toString(16).toUpperCase().padStart(4, "0")}`;
    }
    static opB9(bst: ByteStream) {
        return `mov cx,0x${(x86_16.cx = bst.ReadWord()).toString(16).toUpperCase().padStart(4, "0")}`;
    }
    static opBA(bst: ByteStream) {
        return `mov dx,0x${(x86_16.dx = bst.ReadWord()).toString(16).toUpperCase().padStart(4, "0")}`;
    }
    static opBB(bst: ByteStream) {
        return `mov bx,0x${(x86_16.bx = bst.ReadWord()).toString(16).toUpperCase().padStart(4, "0")}`;
    }
    static opBC(bst: ByteStream) {
        return `mov sp,0x${(x86_16.sp = bst.ReadWord()).toString(16).toUpperCase().padStart(4, "0")}`;
    }
    static opBD(bst: ByteStream) {
        return `mov bp,0x${(x86_16.bp = bst.ReadWord()).toString(16).toUpperCase().padStart(4, "0")}`;
    }
    static opBE(bst: ByteStream) {
        return `mov si,0x${(x86_16.si = bst.ReadWord()).toString(16).toUpperCase().padStart(4, "0")}`;
    }
    static opBF(bst: ByteStream) {
        return `mov di,0x${(x86_16.di = bst.ReadWord()).toString(16).toUpperCase().padStart(4, "0")}`;
    }
    static opC3(bst: ByteStream) {
        const ret = x86_16.stack.pop();
        if (typeof ret == 'number') {
            bst.Position = ret;
        } else {
            throw new Error();
        }
        return "ret";
    }
    static opCD(bst: ByteStream) {
        let vcd = bst.ReadByte();
        let code = `int ${vcd.toString(16)}h`;
        let bytesToSkip: number[] | undefined = undefined;
        switch (vcd) {
            case 0x21:
                switch (x86_16.ah) {
                    case 0x09:
                        let begin = x86_16.dx as number;
                        let end = bst.FindFirstByteFrom(begin, 0x24);
                        code += `\n; printf("${bst.ReadStringFromTo(begin, end).replace('\n', '\\n').replace('\r', '\\r')}");`;
                        bytesToSkip = Array(end - begin + 1).keys().map(x => x + begin).toArray();
                        break;
                    case 0x4C:
                        code += `\n; exit(${x86_16.al});`;
                        break;
                    case 'ah':
                        throw new Error();
                    default:
                        break;
                }
                break;
            default:
                break;
        }
        return {
            code,
            bytesToSkip
        };
    }
    static opE8(bst: ByteStream) {
        let displacement = bst.ReadWord();
        if (displacement & 0x8000) {
            displacement -= 0x10000;
        }
        x86_16.stack.push(bst.Position);
        bst.Position += displacement;
        return `call 0x${bst.Position.toString(16).toUpperCase().padStart(4, '0')}`;
    }
    static opF7(bst: ByteStream) {
        const { reg, rm, v, vS } = this.modrmByteHandling(bst);

        let instr = "";
        switch (reg) {
            case 0:
                const imm16 = bst.ReadWord();
                const res = v ? v : x86_16[regNames[rm]];
                if (typeof res != 'number') {
                    throw new Error();
                }
                x86_16.cf = x86_16.of = 0;
                x86_16.zf = res ? 0 : 1;
                x86_16.sf = (res >> 15) as (0 | 1);
                x86_16.pf = (res & 0xFF).toString(2).replace('0', '').length & 0b1 ? 0 : 1;
                instr = `test ${vS ?? regNames[rm]},0x${imm16.toString(16).toUpperCase()}`;
                break;
            case 2:
                v ? bst.ReplaceWord(v, ~bst.ReadWordAt(v) & 0xFFFF) : x86_16[regNames[rm]] = ~x86_16[regNames[rm]] & 0xFFFF;
                instr = `not ${vS ?? regNames[rm]}`;
                break;
            case 3:
                
                instr = `neg ${vS ?? regNames[rm]}`;
                break;
            case 4:
                instr = `mul ${vS ?? regNames[rm]}`;
                break;
            case 5:
                instr = `imul ${vS ?? regNames[rm]}`;
                break;
            case 6:
                instr = `div ${vS ?? regNames[rm]}`;
                break;
            case 7:
                instr = `idiv ${vS ?? regNames[rm]}`;
                break;
            default:
                throw new Error();
        }

        return instr;
    }

    //#endregion

    static parseCode(bytes: number[]) {
        const bst = new ByteStream(Buffer.from(bytes));

        let code: string[] = [];
        let bytesToSkip: number[] = [];
        let curRep: { op: 'rep' | 'repne', offset: number } | undefined = undefined;

        while (bst.Available()) {
            const debugByte = bst.ReadByte();
            if (bytesToSkip.includes(bst.Position - 1)) {
                continue;
            }

            switch (debugByte) {
                case 0x00:
                    code.push(this.op00());
                    break;
                case 0x0E:
                    code.push(this.op0E());
                    break;
                case 0x1F:
                    code.push(this.op1F());
                    break;
                case 0x33:
                    code.push(this.op33(bst));
                    break;
                case 0x50:
                    code.push(this.op50());
                    break;
                case 0x55:
                    code.push(this.op55());
                    break;
                case 0x56:
                    code.push(this.op56());
                    break;
                case 0x5D:
                    code.push(this.op5D());
                    break;
                case 0x81:
                    code.push(this.op81(bst));
                    break;
                case 0x83:
                    code.push(this.op83(bst));
                    break;
                case 0x8B:
                    code.push(this.op8B(bst));
                    break;
                case 0x8C:
                    code.push(this.op8C(bst));
                    break;
                case 0x8D:
                    code.push(this.op8D(bst));
                    break;
                case 0x8E:
                    code.push(this.op8E(bst));
                    break;
                case 0xAE:
                    code.push(this.opAE(bst));
                    break;
                case 0xB0:
                    code.push(this.opB0(bst));
                    break;
                case 0xB1:
                    code.push(this.opB1(bst));
                    break;
                case 0xB2:
                    code.push(this.opB2(bst));
                    break;
                case 0xB3:
                    code.push(this.opB3(bst));
                    break;
                case 0xB4:
                    code.push(this.opB4(bst));
                    break;
                case 0xB5:
                    code.push(this.opB5(bst));
                    break;
                case 0xB6:
                    code.push(this.opB6(bst));
                    break;
                case 0xB7:
                    code.push(this.opB7(bst));
                    break;
                case 0xB8:
                    code.push(this.opB8(bst));
                    break;
                case 0xB9:
                    code.push(this.opB9(bst));
                    break;
                case 0xBA:
                    code.push(this.opBA(bst));
                    break;
                case 0xBB:
                    code.push(this.opBB(bst));
                    break;
                case 0xBC:
                    code.push(this.opBC(bst));
                    break;
                case 0xBD:
                    code.push(this.opBD(bst));
                    break;
                case 0xBE:
                    code.push(this.opBE(bst));
                    break;
                case 0xBF:
                    code.push(this.opBF(bst));
                    break;
                case 0xC3:
                    code.push(this.opC3(bst));
                    break;
                case 0xCD:
                    const _r = this.opCD(bst);
                    code.push(..._r.code.split('\n'));
                    if (_r.bytesToSkip) {
                        bytesToSkip.push(..._r.bytesToSkip);
                    }
                    break;
                case 0xE8:
                    code.push(this.opE8(bst));
                    break;
                case 0xF2:
                    curRep = {
                        offset: bst.Position,
                        op: 'repne'
                    };
                    continue;
                case 0xF3:
                    curRep = {
                        offset: bst.Position,
                        op: 'rep'
                    };
                    continue;
                case 0xF7:
                    code.push(this.opF7(bst));
                    break;
                default:
                    debugger;
                    break;
            }

            if (curRep) {
                if (curRep.op == 'repne') {
                    if (typeof x86_16.cx != 'number') {
                        throw new Error();
                    }
                    x86_16.cx--;

                    if (x86_16.zf == 1 || x86_16.cx == 0) {
                        curRep = undefined;
                    } else {
                        bst.Position = curRep.offset;
                    }
                }
            }
        }

        return code;
    }
}