mod execute_batch_sc;
mod sc_generation;

fn main() {
    let nb_scs = 1;
    let op_datastore = sc_generation::generate_scs(nb_scs, 10);
    execute_batch_sc::execute_batch_sc(0, nb_scs, op_datastore);
}
