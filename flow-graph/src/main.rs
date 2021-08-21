use clap::clap_app;
use petgraph::dot::{Config as DotConfig, Dot};

use ppc750cl::{disasm_iter, Ins};

pub mod flow;
pub mod slices;

use crate::flow::FlowGraph;
use crate::slices::BasicSlices;

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

    // Create control flow graph.
    let ins_list: Vec<Ins> = disasm_iter(&bytes, addr).collect();
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
