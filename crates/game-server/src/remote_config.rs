use std::time::Duration;

use common::config::*;
use evelyn_http_client::AutopatchClient;
use serde::Deserialize;

use crate::GameServerConfig;

pub struct RemoteConfiguration {
    pub version_info: ConfigurationInfo,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileEntry {
    pub remote_name: String,
    #[expect(unused)]
    pub md5: String,
    #[expect(unused)]
    pub file_size: u64,
    #[expect(unused)]
    pub is_patch: bool,
    #[serde(default)]
    pub tags: Vec<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataVersion {
    #[expect(unused)]
    pub remote_parent_dir: String,
    pub files: Vec<FileEntry>,
}

const RETRY_TIME: Duration = Duration::from_secs(5);

pub fn download_env_config(autopatch_url: &'static str) -> EnvironmentConfiguration {
    let client = AutopatchClient::new(&autopatch_url).retry_after(RETRY_TIME);
    client.fetch_until_success("environment.json")
}

pub fn download_design_data_blk(version_info: &ConfigurationInfo) -> Box<[u8]> {
    let url = format!(
        "{}/{}/{}/",
        version_info.design_data_url, version_info.platform, version_info.environment
    );

    let client = AutopatchClient::new(&url).retry_after(RETRY_TIME);
    let data_version: DataVersion = client.fetch_until_success("data_version");

    let file = data_version
        .files
        .iter()
        .filter(|e| &e.tags == &[2])
        .rev()
        .next()
        .unwrap();

    client.fetch_bytes_until_success(&file.remote_name)
}

pub fn download_main_city_script_config(version_info: &ConfigurationInfo) -> String {
    let url = format!(
        "{}/{}/{}/",
        version_info.design_data_url, version_info.platform, version_info.environment
    );

    let client = AutopatchClient::new(&url).retry_after(RETRY_TIME);
    String::from_utf8_lossy(&client.fetch_bytes_until_success("ServerOnlyData/MainCity_1.json"))
        .to_string()
}

pub fn download(config: &'static GameServerConfig) -> RemoteConfiguration {
    let client = AutopatchClient::new(&config.design_data_url).retry_after(RETRY_TIME);
    let mut app_config: AppConfig = client.fetch_until_success("/config.json");
    let version_info = app_config
        .version_info_groups
        .remove(&config.bind_client_version)
        .expect(
            "Fatal: remote config doesn't contain configuration for specified bind_client_version",
        );

    RemoteConfiguration { version_info }
}
