pub mod processors;

constants_enum! {
    name: StorageClass,
    doc: "",
    value_type: u8,
    items: [
        (Absolute, 0, "The base relocation is skipped. This type can be used to pad a block."),
        (High, 1, "The base relocation adds the high 16 bits of the difference to the 16-bit field at offset. The 16-bit field represents the high value of a 32-bit word."),
        (Low, 2, "The base relocation adds the low 16 bits of the difference to the 16-bit field at offset. The 16-bit field represents the low half of a 32-bit word."),
        (Highlow, 3, "The base relocation applies all 32 bits of the difference to the 32-bit field at offset."),
        (Highadj, 4, "The base relocation adds the high 16 bits of the difference to the 16-bit field at offset. The 16-bit field represents the high value of a 32-bit word. The low 16 bits of the 32-bit value are stored in the 16-bit word that follows this base relocation. This means that this base relocation occupies two slots."),
        //(MipsJmpaddr, 5, "The relocation interpretation is dependent on the machine type.\nWhen the machine type is MIPS, the base relocation applies to a MIPS jump instruction."),
        //(ArmMov32, 5, "This relocation is meaningful only when the machine type is ARM or Thumb. The base relocation applies the 32-bit address of a symbol across a consecutive MOVW/MOVT instruction pair."),
        //(RiscvHigh20, 5, "This relocation is only meaningful when the machine type is RISC-V. The base relocation applies to the high 20 bits of a 32-bit absolute address."),
        //(ThumbMov32, 7, "This relocation is meaningful only when the machine type is Thumb. The base relocation applies the 32-bit address of a symbol to a consecutive MOVW/MOVT instruction pair."),
        //(RiscvLow12I, 7, "This relocation is only meaningful when the machine type is RISC-V. The base relocation applies to the low 12 bits of a 32-bit absolute address formed in RISC-V I-type instruction format."),
        (RiscvLow12S, 8, "This relocation is only meaningful when the machine type is RISC-V. The base relocation applies to the low 12 bits of a 32-bit absolute address formed in RISC-V S-type instruction format."),
        (MipsJmpaddr16, 9, "The relocation is only meaningful when the machine type is MIPS. The base relocation applies to a MIPS16 jump instruction."),
        (Dir64, 10, "The base relocation applies the difference to the 64-bit field at offset."),

    ]
}
