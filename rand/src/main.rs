use rand_core::{RngCore, SeedableRng};
use sfmt::SFMT;

use ppc750cl::formatter::FormattedIns;
use ppc750cl::{Ins, Opcode};

fn main() {
    let mut rng = SFMT::seed_from_u64(42);
    loop {
        let ins = Ins::new(rng.next_u32(), 0);
        if ins.op == Opcode::Illegal {
            continue;
        }
        println!("{}", FormattedIns(ins));
    }
}
