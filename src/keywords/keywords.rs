
pub mod keyword {
    pub const IMPORT: &'static str = "import"; 
    pub const PACKAGE: &'static str = "package"; 
    pub const CIRCLE: &'static str = "circuit"; 
    pub const ELEMENT: &'static str = "element"; 
    pub const FUNCTION: &'static str = "function"; 

    // pub const RESISTOR: &'static str = "Resistor";
    // pub const CAPACITOR: &'static str = "Capacitor"; 
    // pub const INDUCTOR: &'static str = "Inductor"; 
    // pub const VOLTAGE_SOURCE: &'static str = "VoltageSource"; 
    // pub const CURRENT_SOURCE: &'static str = "CurrentSource"; 
}

const ALL_KEYWORDS: [&'static str; 5] = [
    keyword::CIRCLE,
    keyword::ELEMENT,
    keyword::IMPORT,
    keyword::PACKAGE,
    keyword::FUNCTION,
    // keyword::CURRENT_SOURCE,
    // keyword::INDUCTOR,
    // keyword::RESISTOR,
    // keyword::VOLTAGE_SOURCE,
    // keyword::CAPACITOR,
]; 

pub fn is_keyword(word: &str) -> bool {
    ALL_KEYWORDS.contains(&word)
}

