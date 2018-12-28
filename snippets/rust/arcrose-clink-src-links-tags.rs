use std::io::BufRead;
use errors::{ClinkResult, ClinkError};

/// Implements parsing functionality that handles extracting information about tags which are linked to by
/// clink sections and formatted `[clink tag](tag_name)`.
pub struct TagParser<R>
    where R: BufRead
{
    file_path: String,
    file_reader: R
}

/// Contains information parsed out of a clink tag.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tag {
    pub line_number: u64,
    pub file_path: String,
    pub tag_names: Vec<String>
}

impl<R: BufRead> TagParser<R> {
    /// Constructs a new `TagParser` that reads from a specified file.
    pub fn new<T: Into<String>>(path: T, reader: R) -> TagParser<R> {
        TagParser {
            file_path: path.into(),
            file_reader: reader
        }
    }

    /// Parses a file given to the `TagParser` into a vector of `Tags`, denoting locations that can be pointed to by
    /// clink sections.
    ///
    /// ## Algorithm Summary
    ///
    /// 1. Reach each line of the file provided.
    /// 2. In each line, look for an occurrence of "[clink tag]"
    /// 3. When an occurrence is found, parse the tag names provided between parentheses.
    ///   1. Split the string on "," and trim each result.
    /// 4. Return a collection of all the tags found.
    pub fn parse(self) -> ClinkResult<Vec<Tag>> {
        let mut tags: Vec<Tag> = Vec::new();
        for (line_num, line) in self.file_reader.lines().enumerate() {
            let line = line?;
            let line_num = line_num + 1;
            if let Some(open_delim_index) = line.find("[clink tag]") {
                let offset = open_delim_index + "[clink tag]".len();
                let parsed_tag = Tag::parse(line_num as u64, &self.file_path, &line[offset..])?;
                tags.push(parsed_tag);
            }
        }
        Ok(tags)
    }
}

impl Tag {
    /// Parses a list of names from a clink tag.
    ///
    /// # Expected format
    ///
    /// ```
    /// (tag_name[, name2[, ...]])
    /// ```
    fn parse(line_num: u64, file_path: &str, name_section: &str) -> ClinkResult<Tag> {
        let paren_start = name_section 
            .find("(")
            .ok_or(ClinkError::NoTagNames(name_section.to_owned()))?;
        let paren_end = name_section 
            .find(")")
            .ok_or(ClinkError::UnclosedNameSection(name_section.to_owned()))?;
        let name_section = &name_section[paren_start + 1..paren_end];
        let tag_names = name_section
            .split(",")
            .map(str::trim)
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let has_invalid_tag_names = tag_names
            .clone()
            .iter()
            .find(|&name| name == "" || name.contains(" "))
            .is_some();
        if has_invalid_tag_names {
            return Err(ClinkError::BadTagName(name_section.to_owned()));
        }
        Ok(Tag {
            line_number: line_num,
            file_path: file_path.to_owned(),
            tag_names: tag_names,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn tag_parser_handles_single_tag_anchors() {
        let input = "import os
        # [clink tag](main)
        def main():
            print 'Hello world!'";
        let expected_tags = vec![
            Tag {
                line_number: 2,
                file_path: "/main.py".to_owned(),
                tag_names: vec!["main".to_owned()],
            }
        ];
        let parser = TagParser::new("/main.py", Cursor::new(input));
        let tags_found = parser.parse().unwrap();
        assert_eq!(tags_found.len(), expected_tags.len());
        assert_eq!(tags_found[0], expected_tags[0]);
    }

    #[test]
    fn tag_parser_handles_multiple_tag_names() {
        let input = "import os
        def main():
            # [clink tag](first, second)
            print 'Hello world'";
        let expected_tags = vec![
            Tag {
                line_number: 3,
                file_path: "/src/hello.py".to_owned(),
                tag_names: vec![
                    "first".to_owned(),
                    "second".to_owned()
                ]
            }
        ];
        let parser = TagParser::new("/src/hello.py", Cursor::new(input));
        let tags_found = parser.parse().unwrap();
        assert_eq!(tags_found.len(), expected_tags.len());
        assert_eq!(tags_found[0], expected_tags[0]);
    }

    #[test]
    fn tag_parser_handles_multiple_tags() {
        let input = "/// Hello world!
        // [clink tag](hello)
        fn hello() {
            // [clink tag](hello, implementation)
            println!(\"Hello world!\");
        }";
        let expected_tags = vec![
            Tag {
                line_number: 2,
                file_path: "/src/main.rs".to_owned(),
                tag_names: vec!["hello".to_owned()]
            },
            Tag {
                line_number: 4,
                file_path: "/src/main.rs".to_owned(),
                tag_names: vec![
                    "hello".to_owned(),
                    "implementation".to_owned()
                ]
            }
        ];
        let parser = TagParser::new("/src/main.rs", Cursor::new(input));
        let tags_found = parser.parse().unwrap();
        assert_eq!(tags_found.len(), expected_tags.len());
        assert_eq!(tags_found[0], expected_tags[0]);
        assert_eq!(tags_found[1], expected_tags[1]);
    }

    #[test]
    fn tag_parser_errors_on_missing_tag_names() {
        let input = "// [clink tag]()";
        let parser = TagParser::new("/src/main.rs", Cursor::new(input));
        assert!(parser.parse().is_err());
    }

    #[test]
    fn tag_parser_errors_on_unclosed_parens() {
        let input = "// [clink tag](hello";
        let parser = TagParser::new("/src/main.rs", Cursor::new(input));
        assert!(parser.parse().is_err());
    }

    #[test]
    fn tag_parser_skips_malformed_tag() {
        // Notice: click instead of clink
        let input = "// [click tag](hello)";
        let parser = TagParser::new("/src/main.rs", Cursor::new(input));
        let tags_found = parser.parse().unwrap();
        assert_eq!(tags_found.len(), 0);
    }
}