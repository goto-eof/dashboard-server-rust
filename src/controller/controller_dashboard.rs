use std::net::SocketAddr;

use crate::dao::service_dashboard::{retrieve_ip_from_db, update_ip};

use super::controller_common;
use warp::{Rejection, Reply};

pub async fn ping(opt_address: Option<SocketAddr>) -> Result<impl Reply, Rejection> {
    return controller_common::generate_response(Ok(update_ip(opt_address).await));
}

pub async fn retrieve_ip() -> Result<impl Reply, Rejection> {
    return controller_common::generate_response(Ok(retrieve_ip_from_db().await));
}
