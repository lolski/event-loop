use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let el = EventLoop::create();
    el.submit_task(Box::new(|| { println!("hello"); 5 }));
    el.submit_task(Box::new(|| { println!("world"); 5 }));
    el.submit_task(Box::new(|| { println!("!"); 5 }));
    el.stop();
}


enum Event {
    Task(Box<dyn FnOnce() -> i32 + Send>),
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
                let event = recv_chan.recv();
                if event.is_ok() {
                    match event.unwrap() {
                        Event::Task(t) => {
                            let _ = t();
                        }
                        Event::Stop() => {
                            break;
                        }
                    };
                } else {
                    let e: std::sync::mpsc::RecvError = event.err().unwrap();
                    println!("An error occurred: {}", e)
                }
            }
        })
    }

    pub fn submit_task(self: &Self, task: Box<dyn FnOnce() -> i32 + Send>) -> () {
        let _ = self.submit(Event::Task(task));
    }

    fn submit(self: &Self, event: Event) -> () {
        let _ = self.send_chan.send(event);
    }

    pub fn stop(self: Self) -> () {
        let _ = self.submit(Event::Stop());
        let _ = self.loop_thread.join();
    }
}