use crate::database::task::finish_task::finish_task;
use crate::domain::task::TaskStatus;
use crate::routes::tasks::create_task_with_token::CreateTaskRequest;
use block_mesh_common::interfaces::server_api::{
    GetTaskResponse, SubmitTaskRequest, SubmitTaskResponse,
};
use sqlx::PgPool;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ptr::read;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot, Mutex, RwLock};
use tokio::task::JoinHandle;
use uuid::Uuid;

type HttpTask = GetTaskResponse;

#[derive(Debug, Clone)]
pub struct TaskScheduler<T: Debug> {
    task_sender: mpsc::Sender<T>,
    session_sender: mpsc::Sender<NodeController<T>>,
}
impl<T> TaskScheduler<T>
where
    T: Debug + Send + 'static,
{
    pub fn new() -> Self {
        let (task_sender, mut task_receiver) = mpsc::channel(50);
        let (session_sender, mut session_receiver) = mpsc::channel::<NodeController<T>>(1000);
        let scheduler_handle = tokio::spawn(async move {
            while let Some(task) = task_receiver.recv().await {
                // get first ready task
                if let Some(session) = session_receiver.recv().await {
                    // get first ready node / session
                    if let Err(error) = session.task_sender.send(task) {
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
        if let Err(error) = self.session_sender.send(controller).await {
            tracing::error!("Failed to add new session to scheduler");
        }
        Some(task_receiver)
    }

    pub async fn add_task(&self, http_task: T) {
        if let Err(error) = self.task_sender.send(http_task).await {
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
