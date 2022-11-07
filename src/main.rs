use pbr::ProgressBar;

mod calculation;
mod execute_batch_sc;
mod sc_generation;

fn main() {
    let nb_scs = 30;
    let batch_size = 10;
    let op_datastore = sc_generation::generate_op_datastore();
    sc_generation::generate_scs(nb_scs, 100, op_datastore.clone());
    let nb_batch = nb_scs / batch_size;
    let mut pb = ProgressBar::new(nb_batch as u64);
    println!("Executing {} batches", nb_batch);
    for i in  0..nb_batch {
        let duration = execute_batch_sc::execute_batch_sc(i*batch_size, (i+1)*batch_size, op_datastore.clone());
        pb.inc();
    }
    pb.finish_print(&format!("Executed {} batches", nb_batch));
}
