use crate::{
    error::{Error, Result},
    Rva,
};
use core::convert::TryFrom;
use segsource::{DataSegment, TryFromSegment};

impl_section_specifics! { RelocationSection, ".reloc", RelocationBlock }

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error), also_needs(is_32_plus: bool))]
pub struct RelocationBlock {
    /// The image base plus the page RVA is added to each offset to create the VA where the base
    /// relocation must be applied.
    pub page_rva: Rva,

    /// The total number of bytes in the base relocation block, including the Page RVA and Block
    /// Size fields and the Type/Offset fields that follow.
    pub block_size: u32,

    #[from_seg(parse_each, size(remaining))]
    relocations: Vec<Relocation>,
}

#[derive(Debug, Clone)]
pub struct Relocation {
    value: u16,
    pub base_type: BaseRelocationType,
    pub offset: u16,
}

impl<'s> TryFrom<&DataSegment<'s>> for Relocation {
    type Error = Error;

    fn try_from(segment: &DataSegment<'s>) -> Result<Self> {
        let value = segment.next_u16()?;
        let base_type = BaseRelocationType::try_from(value & 0xf000)?;
        let offset = value & 0x0fff;
        Ok(Self {
            value,
            base_type,
            offset,
        })
    }
}

constants_enum! {
    name: BaseRelocationType,
    doc: "",
    value_type: u16,
    items: [
        (Absolute, 0x0000, "The base relocation is skipped. This type can be used to pad a block."),
        (High, 0x1000, "The base relocation adds the high 16 bits of the difference to the 16-bit field at offset. The 16-bit field represents the high value of a 32-bit word."),
        (Low, 0x2000, "The base relocation adds the low 16 bits of the difference to the 16-bit field at offset. The 16-bit field represents the low half of a 32-bit word."),
        (Highlow, 0x3000, "The base relocation applies all 32 bits of the difference to the 32-bit field at offset."),
        (Highadj, 0x4000, "The base relocation adds the high 16 bits of the difference to the 16-bit field at offset. The 16-bit field represents the high value of a 32-bit word. The low 16 bits of the 32-bit value are stored in the 16-bit word that follows this base relocation. This means that this base relocation occupies two slots."),
        (MipsJmpaddr, 0x5000, "The relocation interpretation is dependent on the machine type.\nWhen the machine type is MIPS, the base relocation applies to a MIPS jump instruction."),
        //(ArmMov32, 0x5000, "This relocation is meaningful only when the machine type is ARM or Thumb. The base relocation applies the 32-bit address of a symbol across a consecutive MOVW/MOVT instruction pair."),
        //(RiscvHigh20, 0x5000, "This relocation is only meaningful when the machine type is RISC-V. The base relocation applies to the high 20 bits of a 32-bit absolute address."),
        (ThumbMov32, 0x7000, "This relocation is meaningful only when the machine type is Thumb. The base relocation applies the 32-bit address of a symbol to a consecutive MOVW/MOVT instruction pair."),
        //(RiscvLow12I, 0x7000, "This relocation is only meaningful when the machine type is RISC-V. The base relocation applies to the low 12 bits of a 32-bit absolute address formed in RISC-V I-type instruction format."),
        (RiscvLow12S, 0x8000, "This relocation is only meaningful when the machine type is RISC-V. The base relocation applies to the low 12 bits of a 32-bit absolute address formed in RISC-V S-type instruction format."),
        (MipsJmpaddr16, 0x9000, "The relocation is only meaningful when the machine type is MIPS. The base relocation applies to a MIPS16 jump instruction."),
        (Dir64, 0xa000, "The base relocation applies the difference to the 64-bit field at offset."),
    ]
}
