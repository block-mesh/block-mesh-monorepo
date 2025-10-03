use block_mesh_common::interfaces::server_api::PerkUI;
use std::collections::HashMap;

pub fn get_perks_data() -> HashMap<String, String> {
    let mut perks_data: HashMap<String, String> = HashMap::new();
    perks_data.insert(
        "wallet".to_string(),
        "https://github.com/block-mesh/block-mesh-support-faq/blob/main/CONNECT_WALLET.md"
            .to_string(),
    );
    perks_data.insert(
        "Everlyn_ai".to_string(),
        "https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
            .to_string(),
    );
    perks_data.insert(
        "twitter".to_string(),
        "https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
            .to_string(),
    );
    perks_data.insert(
        "founder_twitter".to_string(),
        "https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
            .to_string(),
    );
    perks_data.insert(
        "xeno_twitter".to_string(),
        "https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
            .to_string(),
    );
    perks_data.insert(
        "wootz_twitter".to_string(),
        "https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
            .to_string(),
    );
    perks_data.insert(
        "proof_of_humanity".to_string(),
        "https://github.com/block-mesh/block-mesh-support-faq/blob/main/PROOF_OF_HUMANITY.md"
            .to_string(),
    );
    perks_data
}

pub fn show_perk(perks: &[PerkUI], name: &str) -> bool {
    !perks.iter().any(|perk| perk.name == name)
}
