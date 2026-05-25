
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("no drawing is available to load")]
    DrawingUnavailable(), 

    #[error("wrong layer pattern: {0}")]
    WrongLayerPattern(String), 
}