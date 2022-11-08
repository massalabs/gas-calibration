use pbr::ProgressBar;

mod calculation;
mod execute_batch_sc;
mod sc_generation;

fn main() {
    let nb_scs = 30;
    let batch_size = 1;
    let op_datastore = sc_generation::generate_op_datastore();
    //sc_generation::generate_scs(nb_scs, 100, op_datastore.clone());
    let nb_batch = nb_scs / batch_size;
    let mut pb = ProgressBar::new(nb_batch as u64);
    println!("Executing {} batches", nb_batch);
    let mut full_stats = Vec::new();
    for i in 0..nb_batch {
        let stats = execute_batch_sc::execute_batch_sc(
            i * batch_size,
            (i + 1) * batch_size,
            op_datastore.clone(),
        );
        full_stats.push(stats);
        pb.inc();
    }
    pb.finish_print(&format!("Executed {} batches", nb_batch));
    calculation::calculate_times(full_stats);
}
