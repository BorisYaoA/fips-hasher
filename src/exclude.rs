use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;

pub struct Excluder {
    matcher: GlobSet,
}

impl Excluder {
    /// Create a new Excluder from a list of glob pattern strings
    pub fn new(patterns: &[String]) -> Result<Self, globset::Error> {
        let mut builder = GlobSetBuilder::new();
        for pat in patterns {
            builder.add(Glob::new(pat)?);
        }
        let matcher = builder.build()?;
        Ok(Excluder { matcher })
    }

    /// Returns true if the given path matches any exclusion pattern
    pub fn is_excluded(&self, path: &Path) -> bool {
        self.matcher.is_match(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exclude_patterns() {
        let excluder = Excluder::new(&vec!["*.log".into(), "temp/*".into()]).unwrap();

        assert!(excluder.is_excluded(Path::new("temp/debug.txt")));
        assert!(excluder.is_excluded(Path::new("error.log")));
        assert!(!excluder.is_excluded(Path::new("src/main.rs")));
    }
}