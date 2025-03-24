import Executable from "./src/reader";

function logInfo(exe: Executable) {
    //#region MZ
    console.log(`MZ Info:`);
    console.log(`  ${exe.MZInfo.LastPageBytes} bytes in the last page`);
    console.log(`  ${exe.MZInfo.PageCount} whole/partial pages (total of ${(exe.MZInfo.PageCount - 1) * 512 + exe.MZInfo.LastPageBytes} bytes)`);
    console.log(`  ${exe.MZInfo.RelocationTableEntryCount} entries in the relocation table`);
    console.log(`  Header size: ${exe.MZInfo.HeaderSize} paragraphs (${exe.MZInfo.HeaderSize * 16} bytes)`);
    console.log(`  Minimum allocation: ${exe.MZInfo.MinimumAllocation} paragraphs (${exe.MZInfo.MinimumAllocation * 16} bytes) required`);
    console.log(`  Maximum allocation: ${exe.MZInfo.MaximumAllocation} paragraphs (${exe.MZInfo.MaximumAllocation * 16} bytes) requested`);
    console.log(`  Initial SS value: ${exe.MZInfo.InitialSS}`);
    console.log(`  Initial SP value: ${exe.MZInfo.InitialSP}`);
    console.log(`  Checksum: ${exe.MZInfo.Checksum}`);
    console.log(`  Initial IP value: ${exe.MZInfo.InitialIP}`);
    console.log(`  Initial CS value: ${exe.MZInfo.InitialCS}`);
    console.log(`  Relocation table offset: 0x${exe.MZInfo.RelocationTableOffset.toString(16).toUpperCase()}${exe.MZInfo.RelocationTableEntryCount == 0 ? " (ignored due to 0 entries)" : ""}`);
    console.log(`  Overlay value: ${exe.MZInfo.Overlay}`);

    console.log();

    if (exe.MZInfo.NewHeaderStart != undefined) {
        console.log("  Windows Executable Extension:");
        console.log(`    OEM Identifier: 0x${exe.MZInfo.OEMIdentifier?.toString(16).toUpperCase()}`);
        console.log(`    OEM Info: 0x${exe.MZInfo.OEMInfo?.toString(16).toUpperCase()}`);
        console.log(`    New header start: 0x${exe.MZInfo.NewHeaderStart?.toString(16).toUpperCase()}`);
    }

    console.log();

    console.log("MZ Header code:");
    exe.MZInfo.HeaderCode.forEach(line => {
        console.log(line);
    });

    console.log();
    //#endregion

    if (exe.Extension) {
        switch (exe.Extension.Signature) {
            case "NE":
                console.log("Windows Executable Signature: New Executable");
                console.log(`  Linker Version: ${exe.Extension.LinkerVersion}`);
                console.log(`  Entry Table Offset: 0x${exe.Extension.EntryTableOffset.toString(16).toUpperCase()}`);
                console.log(`  Entry Table Length: ${exe.Extension.EntryTableLength}`);
                console.log(`  CRC: ${exe.Extension.CRC}`);
                console.log(`  Flag Word: 0x${exe.Extension.FlagWord.toString(16).toUpperCase()}`);
                console.log(`  Automatic Data Segment Number: ${exe.Extension.AutomaticDataSegmentNumber}`);
                console.log(`  Initial Local Heap Size: ${exe.Extension.InitialLocalHeapSize}`);
                console.log(`  Initial Stack Size: ${exe.Extension.InitialStackSize}`);
                console.log(`  CS value: 0x${exe.Extension.CS.toString(16).toUpperCase()}`);
                console.log(`  IP value: 0x${exe.Extension.IP.toString(16).toUpperCase()}`);
                console.log(`  SS value: 0x${exe.Extension.SS.toString(16).toUpperCase()}`);
                console.log(`  SP value: 0x${exe.Extension.SP.toString(16).toUpperCase()}`);
                console.log(`  Entry Count in Segment Table: ${exe.Extension.EntriesInSegmentTable}`);
                console.log(`  Entry count in Module Reference Table: ${exe.Extension.EntiresInModuleReferenceTable}`);
                console.log(`  Non-Resident Names Table Size: ${exe.Extension.NonResidentNamesTableSize}`);
                console.log(`  Segment Table Offset: 0x${exe.Extension.SegmentTableOffset.toString(16).toUpperCase()}`);
                console.log(`  Resource Table Offset: 0x${exe.Extension.ResourceTableOffset.toString(16).toUpperCase()}`);
                console.log(`  Resident Names Table Offset: 0x${exe.Extension.ResidentNamesTableOffset.toString(16).toUpperCase()}`);
                console.log(`  Module Reference Table Offset: 0x${exe.Extension.ModuleReferenceTableOffset.toString(16).toUpperCase()}`);
                console.log(`  Imported Names Table Offset: 0x${exe.Extension.ImportedNamesTableOffset.toString(16).toUpperCase()}`);
                console.log(`  Non Resident Names Table Offset: 0x${exe.Extension.NonResidentNamesTableOffset.toString(16).toUpperCase()}`);
                console.log(`  Moveable Entry Points: ${exe.Extension.MoveableEntryPoints}`);
                console.log(`  Shift Count: ${exe.Extension.ShiftCount}`);
                console.log(`  Resource Segment Count: ${exe.Extension.ResourceSegmentCount}`);
                console.log(`  Target Operating System: ${exe.Extension.TargetOS}`);
                console.log(`  Additional Information: 0x${exe.Extension.AdditionalInfo.toString(16).toUpperCase()}`);
                console.log(`  Fast-Load Offset: 0x${exe.Extension.FastLoadOffset.toString(16).toUpperCase()}`);
                console.log(`  Fast-Load Length: ${exe.Extension.FastLoadLength}`);
                console.log(`  Expected Version: Windows ${exe.Extension.ExpectedWindowsVersion}`);

                console.log();
                break;

            case "PE\0\0":
                break;
        }
    }
}

const file1 = "C:/Users/jjthe/Desktop/16-bit Programs/Spelling Jungle/BST.EXE";
const file2 = "C:/Users/jjthe/Desktop/16-bit Programs/Spelling Jungle/BSTCDRES.DLL";
const file3 = "C:/Users/jjthe/Downloads/American Girls Premiere/DISK1/SETUP.EXE";

console.log("Info of " + file1); logInfo(Executable.Read(file1));
//console.log("Info of " + file2); logInfo(Executable.Read(file2));
console.log("Info of " + file3); logInfo(Executable.Read(file3));
