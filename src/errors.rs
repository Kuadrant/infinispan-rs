use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfinispanError {
    #[error("error while sending the request to Infinispan")]
    Connection(#[from] reqwest::Error),
}
