pub trait Filtered {
    fn layer(&self) -> String;
    fn entity_type(&self) -> String;
}