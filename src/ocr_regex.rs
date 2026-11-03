use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref TIME_FORMAT: Regex = Regex::new(r"(\d{1,2}:\d{2})").unwrap();

    pub static ref TIME_DELTA: Regex = Regex::new(r"([+-]\d{1,2}:\d{2})").unwrap();

    pub static ref DISTANCE_KM: Regex = Regex::new(r"(\d+\.?\d*)\s*[Kk][Mm]").unwrap();

    pub static ref WATTS_PER_KG: Regex = Regex::new(r"(\d+\.\d+)\s*w/kg").unwrap();

    pub static ref DECIMAL_NUMBER: Regex = Regex::new(r"(\d+\.\d+)").unwrap();

    pub static ref NON_DIGITS: Regex = Regex::new(r"[^0-9]").unwrap();

    pub static ref NON_DIGITS_DECIMAL: Regex = Regex::new(r"[^0-9.]").unwrap();

    pub static ref NAME_INITIAL_DOT: Regex = Regex::new(r"^[A-Z]\.[A-Za-z]").unwrap();

    pub static ref SINGLE_LETTER_DOT: Regex = Regex::new(r"^[A-Za-z]\.$").unwrap();
}