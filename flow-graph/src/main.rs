use petgraph::dot::{Config as DotConfig, Dot};

use ppc750cl::{disasm_iter, Ins};

pub mod flow;
pub mod slices;

use crate::flow::FlowGraph;
use crate::slices::BasicSlices;
use dol::Dol;

fn main() {
    let matches = clap::Command::new("ppc750cl-flow-graph")
        .version("0.2.0")
        .about("Control flow graph analysis for PowerPC 750CL")
        .arg(
            clap::Arg::new("START")
                .long("--start")
                .required(true)
                .takes_value(true)
                .help("Start address"),
        )
        .arg(
            clap::Arg::new("STOP")
                .long("--stop")
                .required(true)
                .takes_value(true)
                .help("Stop address"),
        )
        .arg(
            clap::Arg::new("INPUT")
                .required(true)
                .help("Binary input file"),
        )
        .get_matches();

    let start_addr = matches.value_of("START").unwrap();
    let start_addr: u32 = ::parse_int::parse(start_addr).expect("Invalid address flag");
    let stop_addr = matches.value_of("STOP").unwrap();
    let stop_addr: u32 = ::parse_int::parse(stop_addr).expect("Invalid address flag");

    let file_path = matches.value_of("INPUT").unwrap();
    let dol_file = std::fs::File::open(file_path).expect("Failed to read file");
    let dol = Dol::read_from(&dol_file).expect("Invalid DOL file");
    drop(dol_file);
    let mut bytes = vec![0u8; (stop_addr - start_addr) as usize];
    dol.virtual_read(&mut bytes, start_addr);

    // Create control flow graph.
    let ins_list: Vec<Ins> = disasm_iter(&bytes, start_addr).collect();
    let basic_slices = BasicSlices::from_code(&ins_list);
    let graph = FlowGraph::from_basic_slices(&basic_slices, &ins_list);

    // Output graphviz.
    let graphviz = Dot::with_config(
        &graph.graph,
        &[DotConfig::EdgeNoLabel, DotConfig::GraphContentOnly],
    );
    println!(
        concat!(
            "digraph func {{\n",
            "node [shape=record fontname=Arial];\n",
            "{:?}\n",
            "}}"
        ),
        graphviz
    );
}
