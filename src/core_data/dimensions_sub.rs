pub trait DimensionsSubscriber {
    fn update_dimensions(&mut self, cols: u16, rows: u16);
}
