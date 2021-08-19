use clap::clap_app;

use ppc750cl::disasm_iter;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Control flow graph analysis for PowerPC 750CL")
        (@arg ADDR: --addr +required +takes_value "Address")
        (@arg INPUT: +required "Binary input file")
    )
    .get_matches();

    let addr = matches.value_of("ADDR").unwrap();
    let addr: u32 = ::parse_int::parse(addr).expect("Invalid address flag");

    let file_path = matches.value_of("INPUT").unwrap();
    let bytes = std::fs::read(file_path).expect("Failed to read file");

    // disasm_iter(&bytes, addr);
    todo!()
}
