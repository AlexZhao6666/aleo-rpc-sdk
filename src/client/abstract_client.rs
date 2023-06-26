pub trait AbstractClient {
    fn new(base_url:String) -> Self;
    fn get_base_url(&self) -> String;
}
