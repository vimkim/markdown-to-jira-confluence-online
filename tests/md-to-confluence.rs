use markdown_to_confluence::convert_markdown_to_confluence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_headings() {
        let input = "# Heading 1\n## Heading 2\n### Heading 3";
        let expected = "h1. Heading 1\n\nh2. Heading 2\n\nh3. Heading 3\n\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_basic_emphasis() {
        let input = "This is *bold* and _italic_ text";
        let expected = "This is *bold* and _italic_ text\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_nested_lists() {
        let input = "1. First\n   * Nested unordered\n   * Another nested\n2. Second";
        let expected = "\n# First\n* Nested unordered\n* Another nested\n# Second\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_code_blocks_in_lists() {
        let input = "* Item 1\n  ```rust\n  fn main() {}\n  ```\n* Item 2";
        let expected = "\n* Item 1\n{code:language=rust}\nfn main() {}\n{code}\n* Item 2\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_emphasis_in_lists() {
        let input = "* Item with *bold* and _italic_\n* Normal item";
        let expected = "\n* Item with *bold* and _italic_\n* Normal item\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_mixed_nested_lists() {
        let input = "1. First\n   1. Nested numbered\n      * Unordered deep nest\n2. Second";
        let expected = "\n# First\n## Nested numbered\n*** Unordered deep nest\n# Second\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_code_blocks_with_language() {
        let input = "```python\ndef hello():\n    print('Hello')\n```";
        let expected = "\n{code:language=python}\ndef hello():\n    print('Hello')\n{code}\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_plaintext_code_blocks() {
        let input = "```plaintext\nThis is plain text\n```";
        let expected = "\n{code:language=sh}\nThis is plain text\n{code}\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_inline_code() {
        let input = "Use the `println!` macro";
        let expected = "Use the ' {{println!}} ' macro\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_complex_nested_structure() {
        let input = "1. First item\n   * Nested with `code`\n   * Nested with *bold*\n   ```rust\n   fn test() {}\n   ```\n2. Second item\n   > Blockquote\n   * _italic_ text";
        let expected = "\n# First item\n* Nested with ' {{code}} '\n* Nested with *bold*\n{code:language=rust}\nfn test() {}\n{code}\n# Second item\n* _italic_ text\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_horizontal_rules() {
        let input = "Before\n\n---\n\nAfter";
        let expected = "Before\n\n----\n\nAfter\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_html_blocks() {
        let input = "Normal text\n<div>HTML content</div>\nMore text";
        let expected = "Normal text\n{html}<div>HTML content</div>{html}\nMore text\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_empty_lists() {
        let input = "1. \n2. \n* \n* ";
        let expected = "\n# \n# \n* \n* \n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_nested_emphasis() {
        let input = "*Bold _italic_ bold*";
        let expected = " *Bold _italic_ bold* \n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    // Edge cases that might need handling
    #[test]
    fn test_malformed_lists() {
        let input = "1. First\n * Mixed indent\n  * Different indent\n1) Different marker";
        let expected = "\n# First\n* Mixed indent\n* Different indent\n# Different marker\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }

    #[test]
    fn test_code_block_edge_cases() {
        let input = "```\nNo language specified\n```\n    Indented code block";
        let expected =
            "\n{code}\nNo language specified\n{code}\n{code}\nIndented code block\n{code}\n";
        assert_eq!(convert_markdown_to_confluence(input), expected);
    }
}
