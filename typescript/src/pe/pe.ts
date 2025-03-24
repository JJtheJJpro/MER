export class PortableExecutable {
    Signature: "PE\0\0"

    private constructor(magic: "PE\0\0") {
        this.Signature = magic;
    }
}