mod sc_generation;
mod execute_batch_sc;

fn main() {
    let nb_scs = 1;
    sc_generation::generate_scs(nb_scs, 10);
    execute_batch_sc::execute_batch_sc(0, nb_scs);
}
