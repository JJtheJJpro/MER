import { readFileSync } from "fs";
import { MZ } from "./mz";
import { NewExecutable } from "./ne/ne";
import ByteStream from "./bytestream";
import { PortableExecutable } from "./pe/pe";

// NOTES:
// 1 Paragraph = 16 bytes
// 1 page/block/sector = 512 bytes

/**
 * Type used to expand use of the Executable extensions.
 */
export type ExeExtention = NewExecutable | PortableExecutable | undefined;

export class Executable {
    MZInfo: MZ;
    Extension: ExeExtention;

    private constructor(mz: MZ, ext?: ExeExtention) {
        this.MZInfo = mz;
        this.Extension = ext;
    }

    public static Read(file: Buffer): Executable;
    public static Read(file: string): Executable;
    public static Read(file: Buffer | string) {
        if (typeof file === 'string') {
            file = readFileSync(file);
        }
        
        let bst = new ByteStream(file);

        const mz = MZ.Read(bst);
        
        switch (bst.ReadString(2)) {
            case "NE":
                if (mz.NewHeaderStart) {
                    return new Executable(mz, NewExecutable.Read(bst, mz.NewHeaderStart));
                } else {
                    throw new Error();
                }
            default:
                return new Executable(mz);
        }
    }
};

export default Executable;