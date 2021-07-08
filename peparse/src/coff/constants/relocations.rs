use crate::error::Error;
use core::convert::TryFrom;

pub trait CoffRelocationType: TryFrom<u16, Error = Error> {}

constants_enum! {
    name: X86,
    doc: "",
    value_type: u16,
    items: [
        (Absolute, 0x0000, "The relocation is ignored."),
        (Addr64, 0x0001, "The 64-bit VA of the relocation target."),
        (Addr32, 0x0002, "The 32-bit VA of the relocation target."),
        (Addr32Nb, 0x0003, "The 32-bit address without an image base (RVA)."),
        (Rel32, 0x0004, "The 32-bit relative address from the byte following the relocation."),
        (Rel321, 0x0005, "The 32-bit address relative to byte distance 1 from the relocation."),
        (Rel322, 0x0006, "The 32-bit address relative to byte distance 2 from the relocation."),
        (Rel323, 0x0007, "The 32-bit address relative to byte distance 3 from the relocation."),
        (Rel324, 0x0008, "The 32-bit address relative to byte distance 4 from the relocation."),
        (Rel325, 0x0009, "The 32-bit address relative to byte distance 5 from the relocation."),
        (Section, 0x000A, "The 16-bit section index of the section that contains the target. This is used to support debugging information."),
        (Secrel, 0x000B, "The 32-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage."),
        (Secrel7, 0x000C, "A 7-bit unsigned offset from the base of the section that contains the target."),
        (Token, 0x000D, "CLR tokens."),
        (Srel32, 0x000E, "A 32-bit signed span-dependent value emitted into the object."),
        (Pair, 0x000F, "A pair that must immediately follow every span-dependent value."),
        (Sspan32, 0x0010, "A 32-bit signed span-dependent value that is applied at link time."),
    ],
    @markers: CoffRelocationType
}

constants_enum! {
    name: Arm,
    doc: "",
    value_type: u16,
    items: [
        (Absolute, 0x0000, "The relocation is ignored."),
        (Addr32, 0x0001, "The 32-bit VA of the target."),
        (Addr32Nb, 0x0002, "The 32-bit RVA of the target."),
        (Branch24, 0x0003, "The 24-bit relative displacement to the target."),
        (Branch11, 0x0004, "The reference to a subroutine call. The reference consists of two 16-bit instructions with 11-bit offsets."),
        (Rel32, 0x000A, "The 32-bit relative address from the byte following the relocation."),
        (Section, 0x000E, "The 16-bit section index of the section that contains the target. This is used to support debugging information."),
        (Secrel, 0x000F, "The 32-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage."),
        (Mov32, 0x0010, "The 32-bit VA of the target. This relocation is applied using a MOVW instruction for the low 16 bits followed by a MOVT for the high 16 bits."),
        (ThumbMov32, 0x0011, "The 32-bit VA of the target. This relocation is applied using a MOVW instruction for the low 16 bits followed by a MOVT for the high 16 bits."),
        (ThumbBranch20, 0x0012, "The instruction is fixed up with the 21-bit relative displacement to the 2-byte aligned target. The least significant bit of the displacement is always zero and is not stored. This relocation corresponds to a Thumb-2 32-bit conditional B instruction."),
        (ThumbBranch24, 0x0014, "The instruction is fixed up with the 25-bit relative displacement to the 2-byte aligned target. The least significant bit of the displacement is zero and is not stored.This relocation corresponds to a Thumb-2 B instruction."),
        (ThumbBlx23, 0x0015, "The instruction is fixed up with the 25-bit relative displacement to the 4-byte aligned target. The low 2 bits of the displacement are zero and are not stored. This relocation corresponds to a Thumb-2 BLX instruction."),
        (Pair, 0x0016, "The relocation is valid only when it immediately follows a ARM_REFHI or THUMB_REFHI. Its SymbolTableIndex contains a displacement and not an index into the symbol table."),

    ],
    @markers: CoffRelocationType
}

constants_enum! {
    name: Arm64,
    doc: "",
    value_type: u16,
    items: [
        (Absolute, 0x0000, "The relocation is ignored."),
        (Addr32, 0x0001, "The 32-bit VA of the target."),
        (Addr32Nb, 0x0002, "The 32-bit RVA of the target."),
        (Branch26, 0x0003, "The 26-bit relative displacement to the target, for B and BL instructions."),
        (PagebaseRel21, 0x0004, "The page base of the target, for ADRP instruction."),
        (Rel21, 0x0005, "The 12-bit relative displacement to the target, for instruction ADR"),
        (Pageoffset12a, 0x0006, "The 12-bit page offset of the target, for instructions ADD/ADDS (immediate) with zero shift."),
        (Pageoffset12l, 0x0007, "The 12-bit page offset of the target, for instruction LDR (indexed, unsigned immediate)."),
        (Secrel, 0x0008, "The 32-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage."),
        (SecrelLow12A, 0x0009, "Bit 0:11 of section offset of the target, for instructions ADD/ADDS (immediate) with zero shift."),
        (SecrelHigh12A, 0x000A, "Bit 12:23 of section offset of the target, for instructions ADD/ADDS (immediate) with zero shift."),
        (SecrelLow12L, 0x000B, "Bit 0:11 of section offset of the target, for instruction LDR (indexed, unsigned immediate)."),
        (Token, 0x000C, "CLR token."),
        (Section, 0x000D, "The 16-bit section index of the section that contains the target. This is used to support debugging information."),
        (Addr64, 0x000E, "The 64-bit VA of the relocation target."),
        (Branch19, 0x000F, "The 19-bit offset to the relocation target, for conditional B instruction."),
        (Branch14, 0x0010, "The 14-bit offset to the relocation target, for instructions TBZ and TBNZ."),
        (Rel32, 0x0011, "The 32-bit relative address from the byte following the relocation."),

    ],
    @markers: CoffRelocationType
}

constants_enum! {
    name: HitachiSuperH,
    doc: "",
    value_type: u16,
    items: [
        (Sh3Absolute, 0x0000, "The relocation is ignored."),
        (Sh3Direct16, 0x0001, "A reference to the 16-bit location that contains the VA of the target symbol."),
        (Sh3Direct32, 0x0002, "The 32-bit VA of the target symbol."),
        (Sh3Direct8, 0x0003, "A reference to the 8-bit location that contains the VA of the target symbol."),
        (Sh3Direct8Word, 0x0004, "A reference to the 8-bit instruction that contains the effective 16-bit VA of the target symbol."),
        (Sh3Direct8Long, 0x0005, "A reference to the 8-bit instruction that contains the effective 32-bit VA of the target symbol."),
        (Sh3Direct4, 0x0006, "A reference to the 8-bit location whose low 4 bits contain the VA of the target symbol."),
        (Sh3Direct4Word, 0x0007, "A reference to the 8-bit instruction whose low 4 bits contain the effective 16-bit VA of the target symbol."),
        (Sh3Direct4Long, 0x0008, "A reference to the 8-bit instruction whose low 4 bits contain the effective 32-bit VA of the target symbol."),
        (Sh3Pcrel8Word, 0x0009, "A reference to the 8-bit instruction that contains the effective 16-bit relative offset of the target symbol."),
        (Sh3Pcrel8Long, 0x000A, "A reference to the 8-bit instruction that contains the effective 32-bit relative offset of the target symbol."),
        (Sh3Pcrel12Word, 0x000B, "A reference to the 16-bit instruction whose low 12 bits contain the effective 16-bit relative offset of the target symbol."),
        (Sh3StartofSection, 0x000C, "A reference to a 32-bit location that is the VA of the section that contains the target symbol."),
        (Sh3SizeofSection, 0x000D, "A reference to the 32-bit location that is the size of the section that contains the target symbol."),
        (Sh3Section, 0x000E, "The 16-bit section index of the section that contains the target. This is used to support debugging information."),
        (Sh3Secrel, 0x000F, "The 32-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage."),
        (Sh3Direct32Nb, 0x0010, "The 32-bit RVA of the target symbol."),
        (Sh3Gprel4Long, 0x0011, "GP relative."),
        (Sh3Token, 0x0012, "CLR token."),
        (ShmPcrelpt, 0x0013, "The offset from the current instruction in longwords. If the NOMODE bit is not set, insert the inverse of the low bit at bit 32 to select PTA or PTB."),
        (ShmReflo, 0x0014, "The low 16 bits of the 32-bit address."),
        (ShmRefhalf, 0x0015, "The high 16 bits of the 32-bit address."),
        (ShmRello, 0x0016, "The low 16 bits of the relative address."),
        (ShmRelhalf, 0x0017, "The high 16 bits of the relative address."),
        (ShmPair, 0x0018, "The relocation is valid only when it immediately follows a REFHALF, RELHALF, or RELLO relocation. The SymbolTableIndex field of the relocation contains a displacement and not an index into the symbol table."),
        (ShmNomode, 0x8000, "The relocation ignores section mode."),

    ],
    @markers: CoffRelocationType
}

constants_enum! {
    name: IbmPowerPc,
    doc: "",
    value_type: u16,
    items: [
        (Absolute, 0x0000, "The relocation is ignored."),
        (Addr64, 0x0001, "The 64-bit VA of the target."),
        (Addr32, 0x0002, "The 32-bit VA of the target."),
        (Addr24, 0x0003, "The low 24 bits of the VA of the target. This is valid only when the target symbol is absolute and can be sign-extended to its original value."),
        (Addr16, 0x0004, "The low 16 bits of the target's VA."),
        (Addr14, 0x0005, "The low 14 bits of the target's VA. This is valid only when the target symbol is absolute and can be sign-extended to its original value."),
        (Rel24, 0x0006, "A 24-bit PC-relative offset to the symbol's location."),
        (Rel14, 0x0007, "A 14-bit PC-relative offset to the symbol's location."),
        (Addr32Nb, 0x000A, "The 32-bit RVA of the target."),
        (Secrel, 0x000B, "The 32-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage."),
        (Section, 0x000C, "The 16-bit section index of the section that contains the target. This is used to support debugging information."),
        (Secrel16, 0x000F, "The 16-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage."),
        (Refhi, 0x0010, "The high 16 bits of the target's 32-bit VA. This is used for the first instruction in a two-instruction sequence that loads a full address. This relocation must be immediately followed by a PAIR relocation whose SymbolTableIndex contains a signed 16-bit displacement that is added to the upper 16 bits that was taken from the location that is being relocated."),
        (Reflo, 0x0011, "The low 16 bits of the target's VA."),
        (Pair, 0x0012, "A relocation that is valid only when it immediately follows a REFHI or SECRELHI relocation. Its SymbolTableIndex contains a displacement and not an index into the symbol table."),
        (Secrello, 0x0013, "The low 16 bits of the 32-bit offset of the target from the beginning of its section."),
        (Gprel, 0x0015, "The 16-bit signed displacement of the target relative to the GP register."),
        (Token, 0x0016, "The CLR token."),

    ],
    @markers: CoffRelocationType
}

constants_enum! {
    name: I386,
    doc: "",
    value_type: u16,
    items: [
        (Absolute, 0x0000, "The relocation is ignored."),
        (Dir16, 0x0001, "Not supported."),
        (Rel16, 0x0002, "Not supported."),
        (Dir32, 0x0006, "The target's 32-bit VA."),
        (Dir32Nb, 0x0007, "The target's 32-bit RVA."),
        (Seg12, 0x0009, "Not supported."),
        (Section, 0x000A, "The 16-bit section index of the section that contains the target. This is used to support debugging information."),
        (Secrel, 0x000B, "The 32-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage."),
        (Token, 0x000C, "The CLR token."),
        (Secrel7, 0x000D, "A 7-bit offset from the base of the section that contains the target."),
        (Rel32, 0x0014, "The 32-bit relative displacement to the target. This supports the x86 relative branch and call instructions."),

    ],
    @markers: CoffRelocationType
}

constants_enum! {
    name: Itanium,
    doc: "",
    value_type: u16,
    items: [
        (Absolute, 0x0000, "The relocation is ignored."),
        (Imm14, 0x0001, "The instruction relocation can be followed by an ADDEND relocation whose value is added to the target address before it is inserted into the specified slot in the IMM14 bundle. The relocation target must be absolute or the image must be fixed."),
        (Imm22, 0x0002, "The instruction relocation can be followed by an ADDEND relocation whose value is added to the target address before it is inserted into the specified slot in the IMM22 bundle. The relocation target must be absolute or the image must be fixed."),
        (Imm64, 0x0003, "The slot number of this relocation must be one (1). The relocation can be followed by an ADDEND relocation whose value is added to the target address before it is stored in all three slots of the IMM64 bundle."),
        (Dir32, 0x0004, "The target's 32-bit VA. This is supported only for /LARGEADDRESSAWARE:NO images."),
        (Dir64, 0x0005, "The target's 64-bit VA."),
        (Pcrel21B, 0x0006, "The instruction is fixed up with the 25-bit relative displacement to the 16-bit aligned target. The low 4 bits of the displacement are zero and are not stored."),
        (Pcrel21M, 0x0007, "The instruction is fixed up with the 25-bit relative displacement to the 16-bit aligned target. The low 4 bits of the displacement, which are zero, are not stored."),
        (Pcrel21F, 0x0008, "The LSBs of this relocation's offset must contain the slot number whereas the rest is the bundle address. The bundle is fixed up with the 25-bit relative displacement to the 16-bit aligned target. The low 4 bits of the displacement are zero and are not stored."),
        (Gprel22, 0x0009, "The instruction relocation can be followed by an ADDEND relocation whose value is added to the target address and then a 22-bit GP-relative offset that is calculated and applied to the GPREL22 bundle."),
        (Ltoff22, 0x000A, "The instruction is fixed up with the 22-bit GP-relative offset to the target symbol's literal table entry. The linker creates this literal table entry based on this relocation and the ADDEND relocation that might follow."),
        (Section, 0x000B, "The 16-bit section index of the section contains the target. This is used to support debugging information."),
        (Secrel22, 0x000C, "The instruction is fixed up with the 22-bit offset of the target from the beginning of its section. This relocation can be followed immediately by an ADDEND relocation, whose Value field contains the 32-bit unsigned offset of the target from the beginning of the section."),
        (Secrel64I, 0x000D, "The slot number for this relocation must be one (1). The instruction is fixed up with the 64-bit offset of the target from the beginning of its section. This relocation can be followed immediately by an ADDEND relocation whose Value field contains the 32-bit unsigned offset of the target from the beginning of the section."),
        (Secrel32, 0x000E, "The address of data to be fixed up with the 32-bit offset of the target from the beginning of its section."),
        (Dir32Nb, 0x0010, "The target's 32-bit RVA."),
        (Srel14, 0x0011, "This is applied to a signed 14-bit immediate that contains the difference between two relocatable targets. This is a declarative field for the linker that indicates that the compiler has already emitted this value."),
        (Srel22, 0x0012, "This is applied to a signed 22-bit immediate that contains the difference between two relocatable targets. This is a declarative field for the linker that indicates that the compiler has already emitted this value."),
        (Srel32, 0x0013, "This is applied to a signed 32-bit immediate that contains the difference between two relocatable values. This is a declarative field for the linker that indicates that the compiler has already emitted this value."),
        (Urel32, 0x0014, "This is applied to an unsigned 32-bit immediate that contains the difference between two relocatable values. This is a declarative field for the linker that indicates that the compiler has already emitted this value."),
        (Pcrel60X, 0x0015, "A 60-bit PC-relative fixup that always stays as a BRL instruction of an MLX bundle."),
        (Pcrel60B, 0x0016, "A 60-bit PC-relative fixup. If the target displacement fits in a signed 25-bit field, convert the entire bundle to an MBB bundle with NOP.B in slot 1 and a 25-bit BR instruction (with the 4 lowest bits all zero and dropped) in slot 2."),
        (Pcrel60F, 0x0017, "A 60-bit PC-relative fixup. If the target displacement fits in a signed 25-bit field, convert the entire bundle to an MFB bundle with NOP.F in slot 1 and a 25-bit (4 lowest bits all zero and dropped) BR instruction in slot 2."),
        (Pcrel60I, 0x0018, "A 60-bit PC-relative fixup. If the target displacement fits in a signed 25-bit field, convert the entire bundle to an MIB bundle with NOP.I in slot 1 and a 25-bit (4 lowest bits all zero and dropped) BR instruction in slot 2."),
        (Pcrel60M, 0x0019, "A 60-bit PC-relative fixup. If the target displacement fits in a signed 25-bit field, convert the entire bundle to an MMB bundle with NOP.M in slot 1 and a 25-bit (4 lowest bits all zero and dropped) BR instruction in slot 2."),
        (Immgprel64, 0x001a, "A 64-bit GP-relative fixup."),
        (Token, 0x001b, "A CLR token."),
        (Gprel32, 0x001c, "A 32-bit GP-relative fixup."),
        (Addend, 0x001F, "The relocation is valid only when it immediately follows one of the following relocations: IMM14, IMM22, IMM64, GPREL22, LTOFF22, LTOFF64, SECREL22, SECREL64I, or SECREL32. Its value contains the addend to apply to instructions within a bundle, not for data."),

    ],
    @markers: CoffRelocationType
}

constants_enum! {
    name: Mips,
    doc: "",
    value_type: u16,
    items: [
        (Absolute, 0x0000, "The relocation is ignored."),
        (Refhalf, 0x0001, "The high 16 bits of the target's 32-bit VA."),
        (Refword, 0x0002, "The target's 32-bit VA."),
        (Jmpaddr, 0x0003, "The low 26 bits of the target's VA. This supports the MIPS J and JAL instructions."),
        (Refhi, 0x0004, "The high 16 bits of the target's 32-bit VA. This is used for the first instruction in a two-instruction sequence that loads a full address. This relocation must be immediately followed by a PAIR relocation whose SymbolTableIndex contains a signed 16-bit displacement that is added to the upper 16 bits that are taken from the location that is being relocated."),
        (Reflo, 0x0005, "The low 16 bits of the target's VA."),
        (Gprel, 0x0006, "A 16-bit signed displacement of the target relative to the GP register."),
        (Literal, 0x0007, "The same as IMAGE_REL_MIPS_GPREL."),
        (Section, 0x000A, "The 16-bit section index of the section contains the target. This is used to support debugging information."),
        (Secrel, 0x000B, "The 32-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage."),
        (Secrello, 0x000C, "The low 16 bits of the 32-bit offset of the target from the beginning of its section."),
        (Secrelhi, 0x000D, "The high 16 bits of the 32-bit offset of the target from the beginning of its section. An IMAGE_REL_MIPS_PAIR relocation must immediately follow this one. The SymbolTableIndex of the PAIR relocation contains a signed 16-bit displacement that is added to the upper 16 bits that are taken from the location that is being relocated."),
        (Jmpaddr16, 0x0010, "The low 26 bits of the target's VA. This supports the MIPS16 JAL instruction."),
        (Refwordnb, 0x0022, "The target's 32-bit RVA."),
        (Pair, 0x0025, "The relocation is valid only when it immediately follows a REFHI or SECRELHI relocation. Its SymbolTableIndex contains a displacement and not an index into the symbol table."),

    ],
    @markers: CoffRelocationType
}

constants_enum! {
    name: MitsubishiM32R,
    doc: "",
    value_type: u16,
    items: [
        (RAbsolute, 0x0000, "The relocation is ignored."),
        (RAddr32, 0x0001, "The target's 32-bit VA."),
        (RAddr32Nb, 0x0002, "The target's 32-bit RVA."),
        (RAddr24, 0x0003, "The target's 24-bit VA."),
        (RGprel16, 0x0004, "The target's 16-bit offset from the GP register."),
        (RPcrel24, 0x0005, "The target's 24-bit offset from the program counter (PC), shifted left by 2 bits and sign-extended"),
        (RPcrel16, 0x0006, "The target's 16-bit offset from the PC, shifted left by 2 bits and sign-extended"),
        (RPcrel8, 0x0007, "The target's 8-bit offset from the PC, shifted left by 2 bits and sign-extended"),
        (RRefhalf, 0x0008, "The 16 MSBs of the target VA."),
        (RRefhi, 0x0009, "The 16 MSBs of the target VA, adjusted for LSB sign extension. This is used for the first instruction in a two-instruction sequence that loads a full 32-bit address. This relocation must be immediately followed by a PAIR relocation whose SymbolTableIndex contains a signed 16-bit displacement that is added to the upper 16 bits that are taken from the location that is being relocated."),
        (RReflo, 0x000A, "The 16 LSBs of the target VA."),
        (RPair, 0x000B, "The relocation must follow the REFHI relocation. Its SymbolTableIndex contains a displacement and not an index into the symbol table."),
        (RSection, 0x000C, "The 16-bit section index of the section that contains the target. This is used to support debugging information."),
        (RSecrel, 0x000D, "The 32-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage."),
        (RToken, 0x000E, "The CLR token."),

    ],
    @markers: CoffRelocationType
}
