use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread::JoinHandle;
use std::thread;

pub(crate) struct Task {
    task: Box<dyn FnOnce() -> i32 + Send>
}

pub(crate) struct TaskFuture {
    pub future: Receiver<i32>
}

impl TaskFuture {
    pub fn get(self: Self) -> Result<i32, RecvError> {
        self.future.recv()
    }
}

enum TaskExecutorSubmission {
    Task { task: Task, sender_opt: Option<Sender<i32>> },
    StopSignal { sender: Sender<()> }
}

impl TaskExecutorSubmission {
    fn new_task(task: Task) -> (Self, TaskFuture) {
        let (sender, receiver) = channel();
        (TaskExecutorSubmission::Task { task, sender_opt: Some(sender) }, TaskFuture { future: receiver })
    }

    fn new_stop_signal() -> (Self, Receiver<()>) {
        let (sender, receiver) = channel();
        (TaskExecutorSubmission::StopSignal { sender }, receiver )
    }
}

pub(crate) struct TaskExecutor {
    sender: Sender<TaskExecutorSubmission>,
    thread: JoinHandle<()>
}

impl TaskExecutor {
    pub fn create() -> Self {
        let (sender, receiver) = channel();
        let thread = Self::spawn_thread(receiver);
        TaskExecutor { sender, thread }
    }

    fn spawn_thread(recv_chan: Receiver<TaskExecutorSubmission>) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                match recv_chan.recv() {
                    Ok(TaskExecutorSubmission::Task { task, sender_opt: send_chan_opt }) => {
                        let result = (task.task)();
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
                    Ok(TaskExecutorSubmission::StopSignal { sender }) => {
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

    pub fn submit_task(self: &Self, task: Box<dyn FnOnce() -> i32 + Send>) -> TaskFuture {
        let (task_submission, future) = TaskExecutorSubmission::new_task(Task { task });
        let _ = self.submit(task_submission);
        future
    }

    pub fn stop(self: Self) -> () {
        let (stop, recv) = TaskExecutorSubmission::new_stop_signal();
        self.submit(stop);
        recv.recv().unwrap();
        let _ = self.thread.join();
    }

    fn submit(self: &Self, message: TaskExecutorSubmission) -> () {
        let _ = self.sender.send(message);
    }
}
