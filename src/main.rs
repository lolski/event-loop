use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

fn main() {
    let el = EventLoop::create();
    let res = el.submit_task(Box::new(|| { println!("executing task 1"); 5 }));
    let res2 = el.submit_task(Box::new(|| { println!("executing task 2"); 6 }));
    let res3 = el.submit_task(Box::new(|| { println!("executing task 3"); 7 }));
    sleep(Duration::from_secs(10));
    println!("result: {}", res.recv().unwrap());
    println!("result: {}", res2.recv().unwrap());
    println!("result: {}", res3.recv().unwrap());
    el.stop();
}

enum Event {
    Task { task: Box<dyn FnOnce() -> i32 + Send>, sender_opt: Option<Sender<i32>> },
    Stop()
}

struct EventLoop {
    send_chan: Sender<Event>,
    loop_thread: JoinHandle<()>
}

impl EventLoop {
    pub fn create() -> Self {
        let (send_chan, recv_chan) = channel();
        let loop_thread = Self::spawn_event_loop_thread(recv_chan);
        EventLoop { send_chan, loop_thread }
    }

    fn spawn_event_loop_thread(recv_chan: Receiver<Event>) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                match recv_chan.recv() {
                    Ok(Event::Task { task, sender_opt }) => {
                        let result = task();
                        if let Some(sender) = sender_opt {
                            let send = sender.send(result);
                            if let Err(e) = send {
                                println!("An error occurred when sending the result of a task the caller: {}", e);
                            }
                        }
                    },
                    Ok(Event::Stop()) => {
                        break;
                    },
                    Err(e) => {
                        println!("An error occurred: {}", e)
                    }
                }
            }
        })
    }

    pub fn submit_task(self: &Self, task: Box<dyn FnOnce() -> i32 + Send>) -> Receiver<i32> {
        let (send_chan, recv_chan) = channel();
        let _ = self.submit(Event::Task { task, sender_opt: Some(send_chan) });
        recv_chan
    }

    fn submit(self: &Self, event: Event) -> () {
        let _ = self.send_chan.send(event);
    }

    pub fn stop(self: Self) -> () {
        let _ = self.submit(Event::Stop());
        let _ = self.loop_thread.join();
    }
}