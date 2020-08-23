#[repr(u8)]
pub enum OpCode {
    OpConstant,
    OpAdd,
    OpSub,
    OpMult,
    OpDiv,
    OpMod, // modulus operator
    OpNegate,
    OpReturn,
}
