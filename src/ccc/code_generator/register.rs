type RegisterAlias = [&'static str; 4];
// const RAX: RegisterAlias = ["RAX", "EAX", "AX", "AL"];
const RDI: RegisterAlias = ["RDI", "EDI", "DI", "DIL"];
const RSI: RegisterAlias = ["RSI", "ESI", "SI", "SIL"];
const RDX: RegisterAlias = ["RDX", "EDX", "DX", "DL"];
const RCX: RegisterAlias = ["RCX", "ECX", "CX", "CL"];
// const RBP: RegisterAlias = ["RBP", "EBP", "BP", "BPL"];
// const RSP: RegisterAlias = ["RSP", "ESP", "SP", "SPL"];
// const RBX: RegisterAlias = ["RBX", "EBX", "BX", "BL"];
const R8: RegisterAlias = ["R8", "R8D", "R8W", "R8B"];
const R9: RegisterAlias = ["R9", "R9D", "R9W", "R9B"];
// const R10: RegisterAlias = ["R10", "R10D", "R10W", "R10B"];
// const R11: RegisterAlias = ["R11", "R11D", "R11W", "R11B"];
// const R12: RegisterAlias = ["R12", "R12D", "R12W", "R12B"];
// const R13: RegisterAlias = ["R13", "R13D", "R13W", "R13B"];
// const R14: RegisterAlias = ["R14", "R14D", "R14W", "R14B"];
// const R15: RegisterAlias = ["R15", "R15D", "R15W", "R15B"];

/// 関数の引数に使うレジスタ。６個までの引数に対応。
pub const ARGS_REGISTER: [RegisterAlias; 6] = [RDI, RSI, RDX, RCX, R8, R9];
