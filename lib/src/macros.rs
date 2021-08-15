macro_rules! disasm_unreachable {
    ($msg:expr $(,)?) => {{
        panic!(
            "internal error: entered unreachable code disassembling instruction 0x{:08x}",
            $msg
        )
    }};
}
