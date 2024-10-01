#[derive(Debug, PartialEq, Clone)]
pub enum MorseUnit {
    Dot,
    Line,
    EOW, // End Of Word
}

impl MorseUnit {
    pub fn to_beep(&self) {}
}

#[derive(Debug, PartialEq, Clone)]
pub struct MorseDisplayUnit {
    unit: MorseUnit,
    display_as: char,
}

impl MorseDisplayUnit {
    pub fn new(unit: MorseUnit, display_as: char) -> MorseDisplayUnit {
        MorseDisplayUnit { unit, display_as }
    }
}

impl ToString for MorseDisplayUnit {
    fn to_string(&self) -> String {
        match self.unit {
            MorseUnit::Dot => self.display_as.to_string(),
            MorseUnit::Line => self.display_as.to_string(),
            MorseUnit::EOW => self.display_as.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct DisplayAlias {
    pub dot: Option<char>,
    pub line: Option<char>,
    pub eow: Option<char>,
}

impl DisplayAlias {
    pub fn new(dot: Option<char>, line: Option<char>, eow: Option<char>) -> DisplayAlias {
        if dot == line || dot == eow || line == eow {
            panic!("Identical display chars");
        }

        DisplayAlias { dot, line, eow }
    }
}
