mod errors;
use std::{
    fs,
    path::Path,
    str::FromStr,
};
use errors::AocError;

pub struct AocParser {
    pub data: Vec<String>,
}

impl AocParser {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(path)?;
        let data = contents
            .lines()
            .map(String::from)
            .collect();
        Ok(Self {
            data,
        })
    }

    pub fn slice_as_type<T: FromStr>(&self, start: Option<usize>, end: Option<usize>) -> Result<Vec<T>, AocError> {
        let start = start.unwrap_or(0);
        let end = end.unwrap_or(self.data.len());
        if start > end || end > self.data.len() {
            return Err(AocError::OutOfBounds)
        }
        self.data[start..end]
            .iter()
            .map(|s| {
                    s.parse::<T>()
                    .map_err(|_| AocError::ParseToTypeFailed)
                })
            .collect()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let aoc = AocParser::new(".test/input.txt").unwrap();
        assert_eq!(aoc.data.len(), 16);
    }

    #[test]
    fn test_slice_as_type() {
        let input = AocParser {
            data: vec![
                "1 2".to_string(),
                "5 6".to_string(),
                "7 12".to_string(),
                "456 742".to_string(),
                "7442 78524".to_string(),
                "4210 7524".to_string(),
            ],
        };
        let out: Vec<String> = input.slice_as_type(Some(1), Some(5)).unwrap();
        assert_eq!(out.len(), 4);
        assert_eq!(out[0], "5 6".to_string());
        assert_eq!(out[out.len()-1], "7442 78524".to_string());
    }
}
