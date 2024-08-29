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
                if let Some(session) = session_receiver.recv().await {
                    session.task_sender.send(task).unwrap();
                }
            }
        });

        Self {
            task_sender,
            session_sender,
        }
    }

    pub async fn add_session(&self) -> oneshot::Receiver<T> {
        let (task_sender, task_receiver) = oneshot::channel();
        let controller = NodeController::new(task_sender);
        self.session_sender.send(controller).await.unwrap();
        task_receiver
    }

    pub async fn add_task(&self, http_task: T) {
        self.task_sender.send(http_task).await.unwrap();
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

#[tokio::test]
async fn node_workers() {
    let manager = Arc::new(TaskScheduler::new());
    let m = manager.clone();
    let h1 = tokio::spawn(async move {
        let m = m;
        loop {
            let task_receiver = m.add_session().await;
            let task = task_receiver.await.unwrap();
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("Task received A");
            // process task on the client side
        }
    });

    let m = manager.clone();
    let h2 = tokio::spawn(async move {
        let m = m;
        loop {
            let task_receiver = m.add_session().await;
            let task = task_receiver.await.unwrap();
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("Task received B");
            // process the task
        }
    });

    for i in 0..10 {
        manager
            .task_sender
            .send(HttpTask {
                id: Uuid::new_v4(),
                method: String::from("GET"),
                url: String::from("google.com"),
                headers: None,
                body: None,
            })
            .await
            .unwrap();
    }

    tokio::time::sleep(Duration::from_secs(12)).await;
    println!("Assigning task");
}
