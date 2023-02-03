use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let el = EventLoop::create();
    el.submit(Box::new(|| { println!("hello"); 5 }));
    el.stop();
}


struct EventLoop {
    send_chan: Sender<Box<dyn FnOnce() -> i32 + Send>>,
    loop_thread: JoinHandle<()>
}

impl EventLoop {
    pub fn create() -> Self {
        let (send_chan, recv_chan) = channel();
        let loop_thread = Self::spawn_loop_thread(recv_chan);
        EventLoop { send_chan, loop_thread }
    }

    fn spawn_loop_thread(recv_chan: Receiver<Box<dyn FnOnce() -> i32 + Send>>) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let task = recv_chan.recv();
                if task.is_ok() {
                    let t: Box<dyn FnOnce() -> i32> = task.unwrap();
                    let status = t();
                    if status == -666 {
                        break;
                    }
                } else {
                    // let e: RecvError = task.unwrap_err();
                    // println!("An error occurred: {}", e)
                }
            }
        })
    }

    pub fn submit(self: &Self, task: Box<dyn FnOnce() -> i32 + Send>) -> () {
        let _ = self.send_chan.send(task);
    }

    pub fn stop(self: Self) -> () {
        self.submit(Box::new(|| -666));
        let _ = self.loop_thread.join();
    }
}