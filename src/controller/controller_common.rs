use serde::Serialize;
use warp::reply::json;
use warp::{Rejection, Reply};

use super::response::generic_response::GenericResponse;

pub fn generate_response<T: Serialize>(
    data_wrapped: Result<T, Rejection>,
) -> Result<impl Reply, Rejection> {
    let response = match data_wrapped {
        Ok(result) => json::<_>(&GenericResponse {
            status: true,
            result: &result,
        }),
        Err(err) => return Err(err),
    };
    Ok(response)
}
