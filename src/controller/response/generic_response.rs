use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GenericResponse<T> {
    pub status: bool,
    pub result: T,
}
