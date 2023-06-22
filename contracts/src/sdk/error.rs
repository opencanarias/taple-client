use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Serialization Error")]
  SerializationError,
  #[error("Deserialization Error")]
  DeserializationError
}
