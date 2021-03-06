extern crate parcel;
use crate::cpu::Offset;
use parcel::{ParseResult, Parser};

/// Load operand into Accumulator
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct LDA;

impl Offset for LDA {}

impl<'a> Parser<'a, &'a [u8], LDA> for LDA {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], LDA> {
        parcel::one_of(vec![
            parcel::parsers::byte::expect_byte(0xa9),
            parcel::parsers::byte::expect_byte(0xa5),
            parcel::parsers::byte::expect_byte(0xb5),
            parcel::parsers::byte::expect_byte(0xad),
            parcel::parsers::byte::expect_byte(0xbd),
            parcel::parsers::byte::expect_byte(0xb9),
            parcel::parsers::byte::expect_byte(0xa1),
            parcel::parsers::byte::expect_byte(0xb1),
        ])
        .map(|_| LDA)
        .parse(input)
    }
}

/// Load operand into X Register
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct LDX;

impl Offset for LDX {}

impl<'a> Parser<'a, &'a [u8], LDX> for LDX {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], LDX> {
        parcel::one_of(vec![
            parcel::parsers::byte::expect_byte(0xa2),
            parcel::parsers::byte::expect_byte(0xa6),
            parcel::parsers::byte::expect_byte(0xb6),
            parcel::parsers::byte::expect_byte(0xae),
            parcel::parsers::byte::expect_byte(0xbe),
        ])
        .map(|_| LDX)
        .parse(input)
    }
}

/// Load operand into Y Register
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct LDY;

impl Offset for LDY {}

impl<'a> Parser<'a, &'a [u8], LDY> for LDY {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], LDY> {
        parcel::one_of(vec![
            parcel::parsers::byte::expect_byte(0xa0),
            parcel::parsers::byte::expect_byte(0xa4),
            parcel::parsers::byte::expect_byte(0xb4),
            parcel::parsers::byte::expect_byte(0xac),
            parcel::parsers::byte::expect_byte(0xbc),
        ])
        .map(|_| LDY)
        .parse(input)
    }
}

// Store Accumulator in memory
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct STA;

impl Offset for STA {}

impl<'a> Parser<'a, &'a [u8], STA> for STA {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], STA> {
        parcel::one_of(vec![
            parcel::parsers::byte::expect_byte(0x8d),
            parcel::parsers::byte::expect_byte(0x85),
            parcel::parsers::byte::expect_byte(0x95),
            parcel::parsers::byte::expect_byte(0x9d),
            parcel::parsers::byte::expect_byte(0x99),
            parcel::parsers::byte::expect_byte(0x81),
            parcel::parsers::byte::expect_byte(0x91),
        ])
        .map(|_| STA)
        .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct STX;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct STY;

// Arithmetic
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ADC;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SBC;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct INC;

impl Offset for INC {}

impl<'a> Parser<'a, &'a [u8], INC> for INC {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], INC> {
        parcel::parsers::byte::expect_byte(0xee)
            .map(|_| INC)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct INX;

impl Offset for INX {}

impl<'a> Parser<'a, &'a [u8], INX> for INX {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], INX> {
        parcel::parsers::byte::expect_byte(0xe8)
            .map(|_| INX)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct INY;

impl Offset for INY {}

impl<'a> Parser<'a, &'a [u8], INY> for INY {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], INY> {
        parcel::parsers::byte::expect_byte(0xc8)
            .map(|_| INY)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DEC;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DEX;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DEY;

// Shift and Rotate
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ASL;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct LSR;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ROL;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ROR;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AND;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ORA;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct EOR;

// Compare and Test Bit

/// Compare memory with Accumulator
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CMP;

impl Offset for CMP {}

impl<'a> Parser<'a, &'a [u8], CMP> for CMP {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], CMP> {
        parcel::one_of(vec![
            parcel::parsers::byte::expect_byte(0xc9),
            parcel::parsers::byte::expect_byte(0xcd),
            parcel::parsers::byte::expect_byte(0xc5),
            parcel::parsers::byte::expect_byte(0xd5),
            parcel::parsers::byte::expect_byte(0xdd),
            parcel::parsers::byte::expect_byte(0xd9),
            parcel::parsers::byte::expect_byte(0xc1),
            parcel::parsers::byte::expect_byte(0xd1),
        ])
        .map(|_| CMP)
        .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CPX;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CPY;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BIT;

// Branch
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BCC;

impl Offset for BCC {}

impl<'a> Parser<'a, &'a [u8], BCC> for BCC {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], BCC> {
        parcel::one_of(vec![parcel::parsers::byte::expect_byte(0x90)])
            .map(|_| BCC)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BCS;

impl Offset for BCS {}

impl<'a> Parser<'a, &'a [u8], BCS> for BCS {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], BCS> {
        parcel::one_of(vec![parcel::parsers::byte::expect_byte(0xb0)])
            .map(|_| BCS)
            .parse(input)
    }
}

/// Branch on Zero. Follows branch when the Zero flag is not set.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BNE;

impl Offset for BNE {}

impl<'a> Parser<'a, &'a [u8], BNE> for BNE {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], BNE> {
        parcel::parsers::byte::expect_byte(0xd0)
            .map(|_| BNE)
            .parse(input)
    }
}

/// Branch on Zero. Follows branch when the Zero flag is set.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BEQ;

impl Offset for BEQ {}

impl<'a> Parser<'a, &'a [u8], BEQ> for BEQ {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], BEQ> {
        parcel::parsers::byte::expect_byte(0xf0)
            .map(|_| BEQ)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BPL;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BMI;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BVC;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BVS;

// Transfer
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TAX;

impl Offset for TAX {}

impl<'a> Parser<'a, &'a [u8], TAX> for TAX {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], TAX> {
        parcel::parsers::byte::expect_byte(0xaa)
            .map(|_| TAX)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TXA;

impl Offset for TXA {}

impl<'a> Parser<'a, &'a [u8], TXA> for TXA {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], TXA> {
        parcel::parsers::byte::expect_byte(0x8a)
            .map(|_| TXA)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TAY;

impl Offset for TAY {}

impl<'a> Parser<'a, &'a [u8], TAY> for TAY {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], TAY> {
        parcel::parsers::byte::expect_byte(0xa8)
            .map(|_| TAY)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TYA;

impl Offset for TYA {}

impl<'a> Parser<'a, &'a [u8], TYA> for TYA {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], TYA> {
        parcel::parsers::byte::expect_byte(0x98)
            .map(|_| TYA)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TSX;

impl Offset for TSX {}

impl<'a> Parser<'a, &'a [u8], TSX> for TSX {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], TSX> {
        parcel::parsers::byte::expect_byte(0xba)
            .map(|_| TSX)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TXS;

impl Offset for TXS {}

impl<'a> Parser<'a, &'a [u8], TXS> for TXS {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], TXS> {
        parcel::parsers::byte::expect_byte(0x9a)
            .map(|_| TXS)
            .parse(input)
    }
}

// Stack Operations

/// Push Accumulator on stack
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PHA;

impl Offset for PHA {}

impl<'a> Parser<'a, &'a [u8], PHA> for PHA {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], PHA> {
        parcel::parsers::byte::expect_byte(0x48)
            .map(|_| PHA)
            .parse(input)
    }
}

/// Pull Accumulator from stack.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PLA;

impl Offset for PLA {}

impl<'a> Parser<'a, &'a [u8], PLA> for PLA {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], PLA> {
        parcel::parsers::byte::expect_byte(0x46)
            .map(|_| PLA)
            .parse(input)
    }
}

/// Push Processor Status to stack.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PHP;

impl Offset for PHP {}

impl<'a> Parser<'a, &'a [u8], PHP> for PHP {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], PHP> {
        parcel::parsers::byte::expect_byte(0x08)
            .map(|_| PHP)
            .parse(input)
    }
}

// Pull Processor Status from stack.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PLP;

impl Offset for PLP {}

impl<'a> Parser<'a, &'a [u8], PLP> for PLP {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], PLP> {
        parcel::parsers::byte::expect_byte(0x28)
            .map(|_| PLP)
            .parse(input)
    }
}

// Subroutines and Jump

/// Represents a `jmp` instruction, only implemented for the absolute address
/// and indirect modes and functions as jump to a location in memory.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct JMP;

impl Offset for JMP {}

impl<'a> Parser<'a, &'a [u8], JMP> for JMP {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], JMP> {
        parcel::one_of(vec![
            parcel::parsers::byte::expect_byte(0x4c),
            parcel::parsers::byte::expect_byte(0x6c),
        ])
        .map(|_| JMP)
        .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct JSR;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct RTS;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct RTI;

// Set and Clear
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CLC;

impl Offset for CLC {}

impl<'a> Parser<'a, &'a [u8], CLC> for CLC {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], CLC> {
        parcel::parsers::byte::expect_byte(0xad)
            .map(|_| CLC)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SEC;

impl Offset for SEC {}

impl<'a> Parser<'a, &'a [u8], SEC> for SEC {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], SEC> {
        parcel::parsers::byte::expect_byte(0x38)
            .map(|_| SEC)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CLD;

impl Offset for CLD {}

impl<'a> Parser<'a, &'a [u8], CLD> for CLD {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], CLD> {
        parcel::parsers::byte::expect_byte(0xd8)
            .map(|_| CLD)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SED;

impl Offset for SED {}

impl<'a> Parser<'a, &'a [u8], SED> for SED {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], SED> {
        parcel::parsers::byte::expect_byte(0xf8)
            .map(|_| SED)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CLI;

impl Offset for CLI {}

impl<'a> Parser<'a, &'a [u8], CLI> for CLI {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], CLI> {
        parcel::parsers::byte::expect_byte(0x58)
            .map(|_| CLI)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SEI;

impl Offset for SEI {}

impl<'a> Parser<'a, &'a [u8], SEI> for SEI {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], SEI> {
        parcel::parsers::byte::expect_byte(0x78)
            .map(|_| SEI)
            .parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CLV;

impl Offset for CLV {}

impl<'a> Parser<'a, &'a [u8], CLV> for CLV {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], CLV> {
        parcel::parsers::byte::expect_byte(0xb8)
            .map(|_| CLV)
            .parse(input)
    }
}

// Misc
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BRK;

/// Represents a `nop` instruction, only implemented for the implied address
/// mode and functions as a "No Instruction".
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct NOP;

impl Offset for NOP {}

impl<'a> Parser<'a, &'a [u8], NOP> for NOP {
    fn parse(&self, input: &'a [u8]) -> ParseResult<&'a [u8], NOP> {
        parcel::parsers::byte::expect_byte(0xea)
            .map(|_| NOP)
            .parse(input)
    }
}
