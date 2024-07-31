use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub url: String,
    pub id: u64,
    pub name: String,
    pub label: String,
    pub content_type: String,
    pub size: u64,
    pub created_at: String,
    pub browser_download_url: String,
}

impl Asset {
    pub fn json(&self) -> bool {
        self.name.ends_with(".json")
    }

    pub fn target(&self) -> Option<Target> {
        if self.name.ends_with(".dmg") || self.name.ends_with(".app") {
            Some(Target::Darwin)
        } else if self.name.ends_with(".exe") {
            Some(Target::Windows)
        } else if self.name.ends_with(".AppImage") {
            Some(Target::Linux)
        } else {
            None
        }
    }

    pub fn arch(&self) -> Option<Arch> {
        if self.name.contains(Arch::X86_64.to_string().as_str()) {
            Some(Arch::X86_64)
        } else if self.name.contains(Arch::Amd64.to_string().as_str()) {
            Some(Arch::Amd64)
        } else if self.name.contains(Arch::I686.to_string().as_str()) {
            Some(Arch::I686)
        } else if self.name.contains(Arch::Aarch64.to_string().as_str()) {
            Some(Arch::Aarch64)
        } else if self.name.contains(Arch::Armv7.to_string().as_str()) {
            Some(Arch::Armv7)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct LatestRelease {
    pub tag_name: String,
    pub name: String,
    pub created_at: String,
    pub url: String,
    pub id: u64,
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CliVersion {
    pub tag_name: String,
    pub name: String,
    pub url: String,
}

impl LatestRelease {
    pub fn get_json(&self) -> Option<Asset> {
        for asset in self.assets.iter() {
            if asset.json() {
                return Some(asset.clone());
            }
        }
        None
    }

    pub fn get_cli(&self) -> Vec<CliVersion> {
        self.assets
            .iter()
            .filter(|asset| asset.name.contains("blockmesh-cli"))
            .map(|asset| CliVersion {
                tag_name: self.tag_name.clone(),
                name: self.name.clone(),
                url: asset.browser_download_url.clone(),
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_assets(&self, arch: Arch, target: Target) -> Vec<Asset> {
        let mut assets: Vec<Asset> = Vec::new();
        for asset in self.assets.iter() {
            let asset_arch = asset.arch();
            let asset_target = asset.target();
            if let Some(asset_arch) = asset_arch {
                if let Some(asset_target) = asset_target {
                    if asset_arch == arch && asset_target == target {
                        assets.push(asset.clone());
                    }
                }
            }
        }
        assets
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Target {
    Linux,
    Windows,
    Darwin,
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Linux => write!(f, "linux"),
            Target::Windows => write!(f, "windows"),
            Target::Darwin => write!(f, "darwin"),
        }
    }
}

impl TryFrom<&str> for Target {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "linux" => Ok(Target::Linux),
            "windows" => Ok(Target::Windows),
            "darwin" => Ok(Target::Darwin),
            _ => Err(format!("Invalid target: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Arch {
    X86_64,
    I686,
    Aarch64,
    Armv7,
    Amd64,
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::X86_64 => write!(f, "x86_64"),
            Arch::Amd64 => write!(f, "amd64"),
            Arch::I686 => write!(f, "i686"),
            Arch::Aarch64 => write!(f, "aarch64"),
            Arch::Armv7 => write!(f, "armv7"),
        }
    }
}

impl TryFrom<&str> for Arch {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x86_64" => Ok(Arch::X86_64),
            "i686" => Ok(Arch::I686),
            "amd64" => Ok(Arch::Amd64),
            "aarch64" => Ok(Arch::Aarch64),
            "armv7" => Ok(Arch::Armv7),
            _ => Err(format!("Invalid arch: {}", value)),
        }
    }
}
