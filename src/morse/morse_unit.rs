pub enum MorseUnit {
    Dot,
    Line,
    EOW, // End Of Word
}

impl MorseUnit {
    pub fn to_beep(&self) {}
}

impl ToString for MorseUnit {
    fn to_string(&self) -> String {
        match self {
            MorseUnit::Dot => "․".to_string(),
            MorseUnit::Line => "⚊".to_string(),
            MorseUnit::EOW => "       ".to_string(), // The space between words is seven units
        }
    }
}
