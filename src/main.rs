use std::thread::{sleep};
use std::time::Duration;
use task_executor::TaskExecutor;

mod task_executor;

fn main() {
    let el = TaskExecutor::create();
    let res = el.submit_task(Box::new(|| { println!("executing task 1"); 5 }));
    let res2 = el.submit_task(Box::new(|| { println!("executing task 2"); 6 }));
    let res3 = el.submit_task(Box::new(|| { println!("executing task 3"); 7 }));
    sleep(Duration::from_secs(10));
    println!("result: {}", res.get().unwrap());
    println!("result: {}", res2.get().unwrap());
    println!("result: {}", res3.get().unwrap());
    el.stop();
}
