#[derive(Clone, Debug)]
pub enum StackEntry {
    Return((u16, u16)),
    Next((u16, u16), String, f64, f64),
}
