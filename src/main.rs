use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

fn main() {
    let el = TaskExecutor::create();
    let res = el.submit_task(Box::new(|| { println!("executing task 1"); 5 }));
    let res2 = el.submit_task(Box::new(|| { println!("executing task 2"); 6 }));
    let res3 = el.submit_task(Box::new(|| { println!("executing task 3"); 7 }));
    sleep(Duration::from_secs(10));
    println!("result: {}", res.recv().unwrap());
    println!("result: {}", res2.recv().unwrap());
    println!("result: {}", res3.recv().unwrap());
    el.stop();
}

enum Message {
    Task { task: Box<dyn FnOnce() -> i32 + Send>, send_chan_opt: Option<Sender<i32>> },
    Stop { send_chan: Sender<()> }
}

struct TaskExecutor {
    send_chan: Sender<Message>,
    execution_thread: JoinHandle<()>
}

impl TaskExecutor {
    pub fn create() -> Self {
        let (send_chan, recv_chan) = channel();
        let execution_thread = Self::spawn_execution_thread(recv_chan);
        TaskExecutor { send_chan, execution_thread }
    }

    fn spawn_execution_thread(recv_chan: Receiver<Message>) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                match recv_chan.recv() {
                    Ok(Message::Task { task, send_chan_opt }) => {
                        let result = task();
                        if let Some(send_chan) = send_chan_opt {
                            let send = send_chan.send(result);
                            if let Err(e) = send {
                                println!("An error occurred when sending the result of a task the caller: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("An error occurred while receiving an event: {}", e)
                    },
                    Ok(Message::Stop { send_chan: sender }) => {
                        let send = sender.send(());
                        if let Err(e) = send {
                            println!("An error occurred when stopping the event loop: {}", e);
                        }
                        break;
                    }
                }
            }
        })
    }

    pub fn submit_task(self: &Self, task: Box<dyn FnOnce() -> i32 + Send>) -> Receiver<i32> {
        let (send_chan, recv_chan) = channel();
        let _ = self.submit(Message::Task { task, send_chan_opt: Some(send_chan) });
        recv_chan
    }

    pub fn stop(self: Self) -> () {
        let (send_chan, recv_chan) = channel();
        self.submit(Message::Stop { send_chan } );
        recv_chan.recv().unwrap();
        let _ = self.execution_thread.join();
    }

    fn submit(self: &Self, event: Message) -> () {
        let _ = self.send_chan.send(event);
    }
}