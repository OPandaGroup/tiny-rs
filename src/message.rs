#[derive(Debug, Clone)]
pub enum AddSth {
    APi(String),
    Path,
}

#[derive(Debug, Clone)]
pub enum ClearSth {
    APi,
    Path,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddSth(AddSth),
    ClearPath,
    Convert,
    WarnText(String),
}
