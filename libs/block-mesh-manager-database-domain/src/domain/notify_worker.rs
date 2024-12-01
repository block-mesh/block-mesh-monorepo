use block_mesh_common::constants::BLOCKMESH_PG_NOTIFY_WORKER;
use block_mesh_common::interfaces::db_messages::DBMessage;
use sqlx::PgPool;

#[tracing::instrument(name = "notify_worker", skip_all)]
pub async fn notify_worker(pool: &PgPool, messages: &[DBMessage]) -> anyhow::Result<()> {
    let to_send: Vec<Vec<DBMessage>> = split_to_size(messages);
    for m in to_send {
        let s = serde_json::to_string(&m)?.replace('\'', "\"");
        let q = format!("NOTIFY {BLOCKMESH_PG_NOTIFY_WORKER} , '{s}'");
        _ = sqlx::query(&q).execute(pool).await;
    }
    Ok(())
}

#[tracing::instrument(name = "split_to_size", skip_all)]
pub fn split_to_size(messages: &[DBMessage]) -> Vec<Vec<DBMessage>> {
    let mut input_messages = messages.to_vec();
    let mut output: Vec<Vec<DBMessage>> = Vec::with_capacity(100);
    let mut current_vector: Vec<DBMessage> = Vec::with_capacity(100);
    let mut sum = 0;
    while let Some(message) = input_messages.pop() {
        let s = match serde_json::to_string(&message) {
            Ok(r) => r.replace('\'', "\""),
            Err(_) => continue,
        };
        if sum + s.len() >= 2048 {
            output.push(current_vector.clone());
            current_vector.clear();
            sum = s.len();
        } else {
            sum += s.len();
        }
        current_vector.push(message);
    }
    if !current_vector.is_empty() {
        output.push(current_vector.clone());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use block_mesh_common::interfaces::db_messages::{CreateDailyStatMessage, DBMessageTypes};
    use tracing_test::traced_test;
    use uuid::Uuid;

    #[test]
    #[traced_test]
    fn test_split_to_size_1() {
        let messages: Vec<DBMessage> =
            vec![DBMessage::CreateDailyStatMessage(CreateDailyStatMessage {
                user_id: Uuid::new_v4(),
                msg_type: DBMessageTypes::CreateDailyStatMessage,
            })];
        let output = split_to_size(&messages);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].len(), 1);
        let total_size = output.iter().fold(0, |acc, x| acc + x.len());
        assert_eq!(1, total_size);
        let mut flattened: Vec<DBMessage> = output.into_iter().flatten().collect();
        flattened.reverse();
        assert_eq!(messages, flattened);
    }

    #[test]
    #[traced_test]
    fn test_split_to_size_10() {
        let mut messages: Vec<DBMessage> = Vec::with_capacity(10);
        for _ in 0..10 {
            messages.push(DBMessage::CreateDailyStatMessage(CreateDailyStatMessage {
                user_id: Uuid::new_v4(),
                msg_type: DBMessageTypes::CreateDailyStatMessage,
            }));
        }
        let output = split_to_size(&messages);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].len(), 10);
        let total_size = output.iter().fold(0, |acc, x| acc + x.len());
        assert_eq!(10, total_size);
        let mut flattened: Vec<DBMessage> = output.into_iter().flatten().collect();
        flattened.reverse();
        assert_eq!(messages, flattened);
    }

    #[test]
    #[traced_test]
    fn test_split_to_size_100() {
        let mut messages: Vec<DBMessage> = Vec::with_capacity(100);
        for _ in 0..100 {
            messages.push(DBMessage::CreateDailyStatMessage(CreateDailyStatMessage {
                user_id: Uuid::new_v4(),
                msg_type: DBMessageTypes::CreateDailyStatMessage,
            }));
        }
        assert_eq!(messages.len(), 100);
        let output = split_to_size(&messages);
        assert_eq!(output.len(), 6);
        assert_eq!(output[0].len(), 18);
        assert_eq!(output[1].len(), 18);
        assert_eq!(output[2].len(), 18);
        assert_eq!(output[3].len(), 18);
        assert_eq!(output[4].len(), 18);
        assert_eq!(output[5].len(), 10);
        let total_size = output.iter().fold(0, |acc, x| acc + x.len());
        assert_eq!(100, total_size);
        let mut flattened: Vec<DBMessage> = output.into_iter().flatten().collect();
        flattened.reverse();
        assert_eq!(messages, flattened);
    }
}
