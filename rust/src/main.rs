use executable::{mz::{ne::EntryType, MZExtSignature}, Signature};

pub mod apis;
pub mod byte_operation;
pub mod byte_stream;
pub mod executable;

// 1 Paragraph = 16 bytes
// 1 page/block/sector = 512 bytes

fn log_info(file_name: String, exe: Signature) {
    println!("{file_name} Info:");

    match exe {
        Signature::MZ(mz) => {
            println!("MZ (Mark Zbikowski) Executable");

            let page_bytes = if mz.last_page_bytes == 0 {
                mz.page_count * 512
            } else {
                (mz.page_count - 1) * 512 + mz.last_page_bytes
            };
            println!("Page byte count: {page_bytes} bytes");

            if mz.relocation_table_entry_count > 0 && mz.relocation_tables.len() > 0 {
                println!(
                    "Relocation table entries offset: 0x{:04X}",
                    mz.relocation_table_offset
                );
                println!("Relocation table entries:");
                for i in 0..mz.relocation_tables.len() {
                    let reloc_table = &mz.relocation_tables[i];
                    println!(
                        "  Table {i} -> {:04X}:{:04X}",
                        reloc_table.segment, reloc_table.offset
                    );
                }
            } else {
                println!("Relocation table entries: None");
            }

            println!("Header size: {} bytes", mz.header_size * 16);
            println!("Minimum allocation: {} bytes", mz.min_alloc);
            println!("Maximum allocation: {} bytes", mz.max_alloc);
            println!("Registry SP initial value: {}", mz.init_sp);
            println!("Registry SS initial value: {}", mz.init_ss);
            println!("Checksum: {}", mz.checksum);
            println!("Registry CS initial value: {}", mz.init_cs);
            println!("Registry IP initial value: {}", mz.init_ip);
            println!("Overlay: {}", mz.overlay);
            if mz.oem_id != 0 {
                println!("OEM ID: {}", mz.oem_id);
            }
            if mz.oem_info != 0 {
                println!("OEM Info: {}", mz.oem_info);
            }

            if mz.new_header_start > 0 {
                println!("Extension information:");
                match mz.extension.unwrap() {
                    MZExtSignature::NE(ne) => {
                        println!("  NE (New Executable)");
                        println!("  Linker version: {}", ne.linker_version);
                        println!("  CRC: 0x{:08X}", ne.crc);
                        println!("  Flag Info: 0b{:016b}", ne.flag_word);
                        println!(
                            "  Auto data segment number: 0x{:04X}",
                            ne.auto_data_segment_number
                        );
                        println!(
                            "  Initial local heap size: {} bytes",
                            ne.init_local_heap_size
                        );
                        println!("  Initial stack size: {} bytes", ne.init_stack_size);
                        println!("  Registry CS initial value: {}", ne.cs);
                        println!("  Registry IP initial value: {}", ne.ip);
                        println!("  Registry SP initial value: {}", ne.sp);
                        println!("  Registry SS initial value: {}", ne.ss);
                        println!(
                            "  Moveable entry points in entry table: {}",
                            ne.moveable_entry_points
                        );
                        println!("  Shift count: {}", ne.shift_count);
                        println!(
                            "  Number of resource entries: {}",
                            ne.resource_segment_count
                        );
                        print!("  Target OS: {:02X}h", ne.target_os);
                        match ne.target_os {
                            0x02 => println!(" (Windows)"),
                            _ => println!(),
                        }

                        println!();
                        println!("  Table Information:");
                        if ne.entries_in_segment_table > 0 && ne.segments.len() > 0 {
                            println!("    Segment tables:");
                            for i in 0..ne.segments.len() {
                                let segment = &ne.segments[i];
                                println!("      Segment Table {i}:");
                                println!("        Length: {} bytes", segment.length);
                                println!("        Offset: 0x{:04X}", segment.data_offset);
                                println!("        Flags: 0b{:016b}", segment.flags);
                                println!(
                                    "        Minimum allocation size: {} bytes",
                                    segment.min_alloc_size
                                );
                            }
                        } else {
                            println!("    Segment tables: None");
                        }

                        println!("    Resource table:");
                        println!("      Align shift: {}", ne.resource_table.align_shift);
                        if ne.resource_table.types.len() > 0 {
                            println!("      Types:");
                            for i in 0..ne.resource_table.types.len() {
                                let rtype = &ne.resource_table.types[i];
                                println!("        Type info {i}:");
                                println!("          ID or offset: 0x{:04X}", rtype.type_id_or_offset);
                                println!("          Resource count: {}", rtype.res_count);
                                for i2 in 0..rtype.name_info.len() {
                                    let ninfo = &rtype.name_info[i2];
                                    println!("          Name info {i2}:");
                                    println!("            Length: {} bytes", ninfo.length);
                                    println!("            Offset: 0x{:04X}", ninfo.offset);
                                    println!("            ID: {}", ninfo.id);
                                    println!("            Flags: 0b{:016b}", ninfo.flags);
                                }
                            }
                        } else {
                            println!("      Types: None");
                        }
                        if ne.resource_table.resource_strings.len() > 0 {
                            println!("      Resource strings:");
                            for i in 0..ne.resource_table.resource_strings.len() {
                                let rstr = &ne.resource_table.resource_strings[i];
                                println!("        String {i}: {rstr}");
                            }
                        } else {
                            println!("      Resource names: None")
                        }

                        if ne.resident_names.len() > 0 {
                            println!("    Resident name info:");
                            for i in 0..ne.resident_names.len() {
                                let resname = &ne.resident_names[i];
                                println!("      Resident {i}:");
                                println!("        Length: {}", resname.length);
                                println!("        Text: {}", resname.text);
                                println!("        Ordinal: 0x{:04X}", resname.ordinal);
                            }
                        } else {
                            println!("    Resident name info: None");
                        }

                        if ne.module_references.len() > 0 {
                            println!("    Module references info:");
                            for i in 0..ne.module_references.len() {
                                let modref = &ne.module_references[i];
                                println!("      Module {i} offset: 0x{:04X}", modref.offset);
                            }
                        } else {
                            println!("    Module references info: None")
                        }

                        if ne.imported_names.len() > 0 {
                            println!("    Imported names info:");
                            for i in 0..ne.imported_names.len() {
                                let import = &ne.imported_names[i];
                                println!("      Import {i}:");
                                println!("        Length: {}", import.length);
                                println!("        Text: {}", import.text);
                            }
                        } else {
                            println!("    Import names info: None");
                        }

                        if ne.entry_tables.len() > 0 {
                            println!("    Entry tables:");
                            for i in 0..ne.entry_tables.len() {
                                let entry_table = &ne.entry_tables[i];
                                println!("      Entry table {i}:");
                                println!("        Count: {}", entry_table.entry_count);
                                //println!("        Segment Indicator: 0x{:02X}", entry_table.seg_indicator);
                                print!("        Entry Type: ");
                                match entry_table.entry_type {
                                    EntryType::Unused => println!("Unused"),
                                    EntryType::Fixed { flag_word, offset } => {
                                        println!("Fixed");
                                        println!("          Flag word: 0b{:08b}", flag_word);
                                        println!("          Offset: 0x{:04X}", offset);
                                    },
                                    EntryType::Moveable { flag_word, seg_num, offset } => {
                                        println!("Moveable");
                                        println!("          Flag word: 0b{:08b}", flag_word);
                                        println!("          Segment number: {}", seg_num);
                                        println!("          Offset: 0x{:04X}", offset);
                                    }
                                }
                            }
                        } else {
                            println!("    Entry tables: None");
                        }
                    }
                    _ => {}
                }
            } else {
                println!("No MZ extension found");
            }
        }
        _ => {}
    }

    println!();
}

fn mz_ne_test() {
    const FILE1: &str = "../ref/BST.EXE";
    //const FILE2: &str = "../ref/BSTCDRES.DLL";
    //const FILE3: &str = "../ref/SETUPBST.EXE";
    //const FILE4: &str = "../ref/SETUP.EXE";
    println!("Testing MZ NE-extended files...");
    //log_info(executable::read(FILE1).unwrap());
    match executable::read(FILE1) {
        Ok(exe) => log_info(String::from("BST.EXE"), exe),
        Err(e) => eprintln!("BST.EXE Fail: {e}"),
    }
    ////log_info(executable::read(FILE2).unwrap());
    //match executable::read(FILE2) {
    //    Ok(exe) => log_info(String::from("BSTCDRES.DLL"), exe),
    //    Err(e) => eprintln!("BSTCDRES.DLL Fail: {e}"),
    //}
    ////log_info(executable::read(FILE3).unwrap());
    //match executable::read(FILE3) {
    //    Ok(exe) => log_info(String::from("SETUPBST.EXE"), exe),
    //    Err(e) => eprintln!("SETUPBST.EXE Fail: {e}"),
    //}
    ////log_info(executable::read(FILE4).unwrap());
    //match executable::read(FILE4) {
    //    Ok(exe) => log_info(String::from("SETUP.EXE"), exe),
    //    Err(e) => eprintln!("SETUP.EXE Fail: {e}"),
    //}
}

fn mz_pe_test() {
    const FILE1: &str = "../ref/songplayer.exe";
    println!("Testing MZ PE-extended files...");
    //log_info(executable::read(FILE1).unwrap());
    match executable::read(FILE1) {
        Ok(_) => println!("songplayer.exe Success"),
        Err(e) => eprintln!("songplayer.exe Fail: {e}"),
    }
}

fn main() {
    mz_ne_test();
    mz_pe_test();
}
