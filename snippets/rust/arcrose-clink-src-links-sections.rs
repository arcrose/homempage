use std::io::BufRead;
use errors::{ClinkResult, ClinkError};

/// Implements parsing functionality that handles extracting information about sections delimited
/// by `[clink open]()` and `[clink close]()`.
pub struct SectionParser<R>
    where R: BufRead
{
    file_path: String,
    file_reader: R
}

/// Denotes the location of a section encapsulated by `[clink open]()` and `[clink close]()` delimiters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub tags: Vec<String>,
    pub file_path: String,
    open_line_number: u64,
    close_line_number: u64
}

impl<R> SectionParser<R>
    where R: BufRead
{
    /// Construct a new `SectionParser` that will parse a readable source file.
    pub fn new<T: Into<String>>(path: T, reader: R) -> Self {
        SectionParser {
            file_path: path.into(),
            file_reader: reader
        }
    }

    /// Parses the file given to the `SectionParser` into a vector of `Section`s, denoting the
    /// location of parts of the file that need to be monitored for changes.
    ///
    /// ## Algorithm Summary
    ///
    /// 1. Reach each line of the file provided
    /// 2. In each line, look for an occurrence of "[clink open]"
    /// 3. When an occurrence is found, create a new `Section` and extract the text between the following parens.
    ///   1. If there is not a closed pair of parens after "[clink open]", return an error.
    /// 4. Tokenize the text by splitting on "," and stripping leading & trailing whitespace from each token.
    /// 5. Split each token on "#" and trim each part. The results are the tags.
    /// 6. When a "[clink close]()" is encountered, terminate the last `Section` and append it to the results.
    /// 7. Repeat steps 3 to 6 until the file has been completely scanned.
    ///   1. If there are unterminated `Section`s remaining, return an error.
    pub fn parse(self) -> ClinkResult<Vec<Section>> {
        let mut sections: Vec<Section> = Vec::new();
        let mut stack: Vec<Section> = Vec::new();
        for (line_num, line) in self.file_reader.lines().enumerate() {
            let line = line?;
            let line_num = line_num + 1;
            if let Some(open_delim_index) = line.find("[clink open]") {
                let offset = open_delim_index + "[clink open]".len();
                let tags = parse_tags(&line[offset..])?;
                if tags.len() == 0 {
                    return Err(ClinkError::NoLinkSection(line));
                }
                let mut new_section = Section::start(self.file_path.clone(), line_num as u64);
                new_section.tags.extend_from_slice(&tags);
                stack.push(new_section);
            }
            if let Some(_) = line.find("[clink close]()") {
                let mut last_section = stack
                    .pop()
                    .ok_or(ClinkError::ExtraClose(self.file_path.clone(), line_num))?;
                last_section.set_end(line_num as u64);
                sections.push(last_section);
            }
        }
        if stack.len() > 0 {
            Err(ClinkError::UnclosedSections(self.file_path.clone(), stack.len()))
        } else {
            Ok(sections)
        }
    }
}

impl Section {
    /// Create a new `Section` that starts on a given line of a file.
    ///
    /// Will be initialized with a closing line number equal to the opening line number and an empty set of links.
    pub fn start(path: String, open_line_num: u64) -> Section {
        Section {
            file_path: path,
            open_line_number: open_line_num,
            close_line_number: open_line_num,
            tags: vec![]
        }
    }

    /// Sets the line number at which a section ends.
    pub fn set_end(&mut self, end_line_num: u64) {
        self.close_line_number = end_line_num;
    }

    /// Checks if a section contains any of a list of line numbers.
    pub fn covers_any_line(&self, line_nums: &Vec<u32>) -> bool {
        for &line_num in line_nums.iter() {
            let line_num = line_num as u64;
            if line_num >= self.open_line_number && line_num <= self.close_line_number {
                return true;
            }
        }
        false
    }
}

/// Parses a tag section containing one or more comma-separated tags.
///
/// # Expected Format
///
/// ```
/// (tag_name[, tag_name[, ...]])
/// ```
pub fn parse_tags(tag_section: &str) -> ClinkResult<Vec<String>> {
    let mut tags : Vec<String> = Vec::new();
    let tag_section = tag_section.trim();
    let paren_start = tag_section
        .find("(")
        .ok_or(ClinkError::NoLinkSection(tag_section.to_owned()))?;
    let paren_end = tag_section[paren_start + 1..]
        .find(")")
        .ok_or(ClinkError::UnclosedLinkSection(tag_section.to_owned()))?;
    let tag_section = &tag_section[paren_start + 1..paren_end+1];
    let tag_parts = tag_section
        .split(",")
        .map(str::trim);
    for tag in tag_parts {
        let tag = tag.trim();
        if tag.contains(" ") || tag.len() == 0 {
            return Err(ClinkError::BadTagName(tag.to_owned()));
        }
        tags.push(tag.to_owned());
    }
    Ok(tags)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn section_parser_handles_simple_sections() {
        let input = "import os
        # [clink open](main)
        def show_process():
            print os.getpid()
        # [clink close]()";
        let expected_sections = vec![
            Section {
                open_line_number: 2,
                close_line_number: 5,
                file_path: "/process.py".to_owned(),
                tags: vec!["main".to_owned()]
            }
        ];
        let parser = SectionParser::new("/process.py", Cursor::new(input));
        let sections_found = parser.parse().unwrap();
        assert_eq!(sections_found.len(), expected_sections.len());
        assert_eq!(sections_found[0], expected_sections[0]);
    }

    #[test]
    fn section_parser_handles_nested_sections() {
        let input = "// [clink open](test)
        fn test() {
            // [clink open](logger)
            println!(\"hello\");
            // [clink close]()
        }
        // [clink close]()";
        let expected_sections = vec![
            Section {
                open_line_number: 3,
                close_line_number: 5,
                file_path: "/src/main.rs".to_owned(),
                tags: vec!["logger".to_owned()]
            },
            Section {
                open_line_number: 1,
                close_line_number: 7,
                file_path: "/src/main.rs".to_owned(),
                tags: vec!["test".to_owned()]
            },
        ];
        let parser = SectionParser::new("/src/main.rs", Cursor::new(input));
        let sections_found = parser.parse().unwrap();
        assert_eq!(sections_found.len(), expected_sections.len());
        assert_eq!(sections_found[0], expected_sections[0]);
        assert_eq!(sections_found[1], expected_sections[1]);
    }

    #[test]
    fn section_parser_handles_multiple_links() {
        let input = "// [clink open](logger)
        println!(\"hello\");
        // [clink close]()";
        let expected_sections = vec![
            Section {
                open_line_number: 1,
                close_line_number: 3,
                file_path: "/tests/mod.rs".to_owned(),
                tags: vec!["logger".to_owned()]
            }
        ];
        let parser = SectionParser::new("/tests/mod.rs", Cursor::new(input));
        let sections_found = parser.parse().unwrap();
        assert_eq!(sections_found.len(), expected_sections.len());
        assert_eq!(sections_found[0], expected_sections[0]);
    }

    #[test]
    fn section_parser_errors_on_missing_links() {
        let input = "## [clink open]()
        # [clink close]()";
        let parser = SectionParser::new("/tests/mod.rs", Cursor::new(input));
        assert!(parser.parse().is_err());
    }

    #[test]
    fn section_parser_errors_on_malformed_links() {
        let input = "// [clink open](/src/main.rs#logger one)
        // [clink close]()";
        let parser = SectionParser::new("/tests/mod.rs", Cursor::new(input));
        assert!(parser.parse().is_err());
    }

    #[test]
    fn section_parser_errors_on_unclosed_parens() {
        let input = "// [clink open](/src/main.rs#logger
        println!(\"hello\");
        // [clink close]()";
        let parser = SectionParser::new("/tests/mod.rs", Cursor::new(input));
        assert!(parser.parse().is_err());
    }

    #[test]
    fn section_parser_error_on_unclosed_delimiters() {
        // notice: click close instead of clink close
        let input = "// [clink open](/src/main.rs)
        // [click close]()";
        let parser = SectionParser::new("/tests/mod.rs", Cursor::new(input));
        assert!(parser.parse().is_err());
    }

    #[test]
    fn section_parser_skips_malformed_delimiters() {
        // notice: click instead of clink
        let input = "// [click open](/hello.py)
        // [clink close]()";
        let parser = SectionParser::new("/tests/mod.rs", Cursor::new(input));
        assert!(parser.parse().is_err());
    }
}