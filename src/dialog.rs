
pub struct Dialog {
    pub show: bool,
    pub title: String,
    pub msg: String
}

impl Default for Dialog {
    fn default() -> Self {
        Self {
            show: false,
            title: String::from(""),
            msg: String::from("")
        }
    }
}