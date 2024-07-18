use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum TauriCommand {
    Invoke,
}
