use crate::controller::controller_dashboard;
use warp::{Filter, Rejection, Reply};

pub fn get_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let routes = warp::path("ping")
        .and(warp::post())
        .and(warp::addr::remote())
        .and(warp::path::end())
        .and_then(controller_dashboard::ping)
        .or(warp::path!("ip")
            .and(warp::get())
            .and(warp::path::end())
            .and_then(controller_dashboard::retrieve_ip));
    return routes;
}
