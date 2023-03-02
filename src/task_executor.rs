use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;
use std::thread;

enum TaskExecutorMessage {
    TaskSubmission { task: Box<dyn FnOnce() -> i32 + Send>, sender_opt: Option<Sender<i32>> },
    Stop { sender: Sender<()> }
}

pub(crate) struct TaskExecutor {
    sender: Sender<TaskExecutorMessage>,
    thread: JoinHandle<()>
}

impl TaskExecutor {
    pub fn create() -> Self {
        let (sender, receiver) = channel();
        let thread = Self::spawn_thread(receiver);
        TaskExecutor { sender, thread }
    }

    fn spawn_thread(recv_chan: Receiver<TaskExecutorMessage>) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                match recv_chan.recv() {
                    Ok(TaskExecutorMessage::TaskSubmission { task, sender_opt: send_chan_opt }) => {
                        let result = task();
                        if let Some(send_chan) = send_chan_opt {
                            let send = send_chan.send(result);
                            if let Err(e) = send {
                                println!("An error occurred when sending the result of a task the caller: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("An error occurred while receiving a task: {}", e)
                    },
                    Ok(TaskExecutorMessage::Stop { sender }) => {
                        let send = sender.send(());
                        if let Err(e) = send {
                            println!("An error occurred when stopping the task executor: {}", e);
                        }
                        break;
                    }
                }
            }
        })
    }

    pub fn submit_task(self: &Self, task: Box<dyn FnOnce() -> i32 + Send>) -> Receiver<i32> {
        let (send_chan, recv_chan) = channel();
        let _ = self.submit(TaskExecutorMessage::TaskSubmission { task, sender_opt: Some(send_chan) });
        recv_chan
    }

    pub fn stop(self: Self) -> () {
        let (send_chan, recv_chan) = channel();
        self.submit(TaskExecutorMessage::Stop { sender: send_chan } );
        recv_chan.recv().unwrap();
        let _ = self.thread.join();
    }

    fn submit(self: &Self, message: TaskExecutorMessage) -> () {
        let _ = self.sender.send(message);
    }
}
