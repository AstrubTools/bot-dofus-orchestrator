#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
pub mod repository;
use mongodb::bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskType {
    LoginToAccount,
    LoginToCharacter,
    Disconnect,
    FeedPets,
    ShopScrap
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskResult {
    Success { log: String },
    Failure { log: String }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub task: TaskType,
    pub parameters: Option<String>,
    pub result: Option<TaskResult>,
    pub date: Option<String>,
    pub state: Option<String> // TO improve everything
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableTask {
    pub task: TaskType,
    pub parameters: Option<String>,
    pub result: Option<TaskResult>,
    pub date: Option<String>,
    pub state: Option<String> // TO improve everything
}

impl InsertableTask {
    fn from_task(task: Task) -> InsertableTask {
        InsertableTask {
            task: task.task,
            parameters: task.parameters,
            result: task.result,
            date: task.date,
            state: task.state
        }
    }
    
    fn assign_id(task: InsertableTask, new_id: bson::oid::ObjectId) -> Task {
        Task {
            id: Some(new_id),
            task: task.task,
            parameters: task.parameters,
            result: task.result,
            date: task.date,
            state: task.state,
        }
    }
}
