use std::fmt::Debug;

use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Clone)]
pub struct TaskScheduler<T: Debug> {
    task_sender: mpsc::Sender<T>,
    session_sender: mpsc::Sender<NodeController<T>>,
}
impl<T> Default for TaskScheduler<T>
where
    T: Debug + Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> TaskScheduler<T>
where
    T: Debug + Send + 'static,
{
    pub fn new() -> Self {
        let (task_sender, mut task_receiver) = mpsc::channel(50);
        let (session_sender, mut session_receiver) = mpsc::channel::<NodeController<T>>(1000);
        let _scheduler_handle = tokio::spawn(async move {
            while let Some(task) = task_receiver.recv().await {
                // get first ready task
                if let Some(session) = session_receiver.recv().await {
                    // get first ready node / session
                    if let Err(_error) = session.task_sender.send(task) {
                        tracing::error!("Assigned node left early");
                        // TODO consider returning this task to the pool at the front
                    }
                }
            }
        });

        Self {
            task_sender,
            session_sender,
        }
    }

    pub async fn add_session(&self) -> Option<oneshot::Receiver<T>> {
        let (task_sender, task_receiver) = oneshot::channel();
        let controller = NodeController::new(task_sender);
        match self.session_sender.send(controller).await {
            Ok(_) => Some(task_receiver),
            Err(e) => {
                tracing::error!("Failed to add new session to scheduler {:?}", e);
                None
            }
        }
    }

    pub async fn add_task(&self, http_task: T) {
        if let Err(_error) = self.task_sender.send(http_task).await {
            tracing::error!("Failed to add new task to scheduler");
        }
    }
}

struct NodeController<T> {
    task_sender: oneshot::Sender<T>,
}

impl<T> NodeController<T> {
    fn new(task_sender: oneshot::Sender<T>) -> Self {
        Self { task_sender }
    }
}
