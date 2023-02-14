use std::sync::mpsc::{channel, Receiver, Sender, sync_channel, SyncSender};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let el = EventLoop::create();
    let res = el.submit_task(Box::new(|| { println!("executing task 1"); 5 }));
    let res2 = el.submit_task(Box::new(|| { println!("executing task 2"); 6 }));
    let res3 = el.submit_task(Box::new(|| { println!("executing task 3"); 7 }));
    println!("result: {}", res.recv().unwrap());
    println!("result: {}", res2.recv().unwrap());
    println!("result: {}", res3.recv().unwrap());
    el.stop();
}

enum Event {
    Task(Box<dyn FnOnce() -> i32 + Send>, Option<SyncSender<i32>>),
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
                        Event::Task(task, s_opt) => {
                            let result = task();
                            if let Some(s) = s_opt {
                                s.send(result).unwrap();
                            }
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

    pub fn submit_task(self: &Self, task: Box<dyn FnOnce() -> i32 + Send>) -> Receiver<i32> {
        let (send_chan, recv_chan) = sync_channel::<i32>(1);
        let _ = self.submit(Event::Task(task, Some(send_chan)));
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