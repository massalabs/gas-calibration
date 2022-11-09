use pbr::ProgressBar;
use rand::Rng;

mod calculation;
mod execute_batch_sc;
mod sc_generation;

fn main() {
    let mut rng = rand::thread_rng();
    let nb_scs = 10000;
    //let op_datastore = sc_generation::read_existing_op_datastore();
    let op_datastore = sc_generation::generate_op_datastore();
    sc_generation::generate_scs(nb_scs, 1000, op_datastore.clone());
    let mut pb = ProgressBar::new(nb_scs as u64);
    println!("Executing {} SCs", nb_scs);
    let mut full_stats = Vec::new();
    let mut executed = 0;
    while executed < nb_scs {
        let nb_exec = rng.gen_range(1..100);
        println!("Executing {} SCs", nb_exec);
        let stats = execute_batch_sc::execute_batch_sc(
            executed,
            std::cmp::min(nb_scs, executed + nb_exec),
            op_datastore.clone(),
            &mut pb,
        );
        executed += nb_exec;
        full_stats.push(stats);
    }
    pb.finish_print(&format!("Executed {} Scs", nb_scs));
    calculation::calculate_times(full_stats);
}
