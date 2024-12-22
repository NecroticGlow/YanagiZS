use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ServiceType {
    #[serde(rename = "dispatch")]
    Dispatch,
    #[serde(rename = "gate-server")]
    GateServer,
    #[serde(rename = "game-server")]
    GameServer,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceConfiguration {
    #[serde(rename = "type")]
    pub ty: ServiceType,
    pub id: u32,
    pub addr: SocketAddr,
}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentConfiguration {
    pub services: Vec<ServiceConfiguration>,
}

impl EnvironmentConfiguration {
    pub fn get_server_end_point(&self, ty: ServiceType, id: u32) -> Option<SocketAddr> {
        self.services
            .iter()
            .find(|svc| svc.ty == ty && svc.id == id)
            .map(|svc| svc.addr)
    }
}
