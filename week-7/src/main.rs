use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::thread;
use std::time::Instant;

fn main() {
    let data: Arc<Vec<i64>> = Arc::new((1..100000001).collect());

    // single thread sum
    let start_time = Instant::now();
    let single_thread_sum: i64 = data.iter().sum();
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("single thread sum | elapsed time = {:?}, sum = {:?}", elapsed_time, single_thread_sum);

    // multi thread sum
    let sum = Arc::new(AtomicI64::new(0));
    let mut handles = Vec::new();

    let start_time = Instant::now();
    for i in 0..10 {
        let start_idx = i * 10000000;
        let data = Arc::clone(&data);
        let sum = Arc::clone(&sum);
        handles.push(thread::spawn(move || {
            let mut partial_sum: i64 = 0;
            for j in start_idx..start_idx + 10000000 {
                partial_sum += data[j]
            }
            sum.fetch_add(partial_sum, Ordering::Relaxed);
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("multi thread sum  | elapsed time = {:?}, sum = {:?}", elapsed_time, sum);
}
