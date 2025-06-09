//! Compiled regex patterns for OCR
//! Using lazy_static to compile once and reuse

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Time format regex: matches HH:MM or H:MM
    pub static ref TIME_FORMAT: Regex = Regex::new(r"(\d{1,2}:\d{2})").unwrap();
    
    /// Time delta regex: matches +HH:MM or -HH:MM
    pub static ref TIME_DELTA: Regex = Regex::new(r"([+-]\d{1,2}:\d{2})").unwrap();
    
    /// Distance regex: matches XX.X KM or XX KM (case insensitive)
    pub static ref DISTANCE_KM: Regex = Regex::new(r"(\d+\.?\d*)\s*[Kk][Mm]").unwrap();
    
    /// W/kg regex: matches X.X w/kg
    pub static ref WATTS_PER_KG: Regex = Regex::new(r"(\d+\.\d+)\s*w/kg").unwrap();
    
    /// Decimal number regex: matches X.X
    pub static ref DECIMAL_NUMBER: Regex = Regex::new(r"(\d+\.\d+)").unwrap();
    
    /// Clean non-digits regex
    pub static ref NON_DIGITS: Regex = Regex::new(r"[^0-9]").unwrap();
    
    /// Clean non-digits and decimal points regex
    pub static ref NON_DIGITS_DECIMAL: Regex = Regex::new(r"[^0-9.]").unwrap();
    
    /// Name pattern: initial with dot followed by name (e.g., J.Chidley)
    pub static ref NAME_INITIAL_DOT: Regex = Regex::new(r"^[A-Z]\.[A-Za-z]").unwrap();
    
    /// Single letter with dot pattern
    pub static ref SINGLE_LETTER_DOT: Regex = Regex::new(r"^[A-Za-z]\.$").unwrap();
}