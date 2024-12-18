use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Parser, Tag, TagEnd};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn convert_markdown_to_confluence(input: &str) -> String {
    let parser = Parser::new(input);
    let mut output = String::new();
    let mut list_depth = 0;
    let mut list_stack: Vec<bool> = Vec::new(); // Stack to track list types (true for ordered, false for unordered)

    for event in parser {
        // let event = dbg!(event);
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                } => {
                    // Convert the heading level to Confluence format (e.g., `h1.`, `h2.`, etc.)
                    let heading_level = match level {
                        HeadingLevel::H1 => "h1.",
                        HeadingLevel::H2 => "h2.",
                        HeadingLevel::H3 => "h3.",
                        HeadingLevel::H4 => "h4.",
                        HeadingLevel::H5 => "h5.",
                        HeadingLevel::H6 => "h6.",
                    };

                    // Start the heading in Confluence format
                    output.push_str(&format!("\n\n{} ", heading_level));

                    // Optionally include `id` if needed
                    if let Some(id_value) = id {
                        output.push_str(&format!("[ID: {}] ", id_value));
                    }

                    // Optionally include `classes` if needed
                    if !classes.is_empty() {
                        output.push_str(&format!("[Classes: {:?}] ", classes));
                    }

                    // Optionally include attributes if needed
                    if !attrs.is_empty() {
                        output.push_str(&format!("[Attrs: {:?}] ", attrs));
                    }
                }
                Tag::Emphasis => output.push('_'),
                Tag::Strong => output.push('*'),
                Tag::List(Some(_)) => {
                    // Ordered list: push `true` to stack
                    list_stack.push(true);
                    list_depth += 1;
                }
                Tag::List(None) => {
                    // Unordered list: push `false` to stack
                    list_stack.push(false);
                    list_depth += 1;
                }
                Tag::Item => {
                    // Use the last value in the stack to determine list type
                    if let Some(&is_ordered_list) = list_stack.last() {
                        if is_ordered_list {
                            output.push_str(&format!("\n{} ", "#".repeat(list_depth)));
                        } else {
                            output.push_str(&format!("\n{} ", "*".repeat(list_depth)));
                        }
                    }
                }
                Tag::CodeBlock(CodeBlockKind::Fenced(lang)) => {
                    let l = if lang.as_ref() == "plaintext" {
                        CowStr::from("sh")
                    } else {
                        lang // Use the original CowStr
                    };
                    output.push_str(&format!("\n{{code:language={}}}\n", l));
                }
                Tag::CodeBlock(CodeBlockKind::Indented) => {
                    // Start a generic code block for indented code
                    output.push_str("\n{code}\n");
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Heading(_) => output.push('\n'),
                TagEnd::Emphasis => output.push('_'),
                TagEnd::Strong => output.push('*'),
                TagEnd::List(_) => {
                    // Pop the stack to restore the previous list type
                    list_stack.pop();
                    list_depth -= 1;
                    if list_depth == 0 {
                        output.push('\n');
                    }
                }
                TagEnd::Item => {
                    // Add a line break after each list item
                }
                TagEnd::CodeBlock => {
                    // Write the Confluence code block end marker
                    output.push_str("{code}");
                }
                _ => {}
            },
            Event::Text(text) => {
                // Add text content
                output.push_str(&text);
            }
            Event::SoftBreak | Event::HardBreak => {
                // Add a line break
                output.push('\n');
            }
            Event::Code(text) => {
                // Inline code
                output.push_str(&format!(" {{{{{}}}}} ", text));
            }
            Event::Html(html) => {
                // Add raw HTML content
                output.push_str(&format!("{{html}}{}{{html}}", html));
            }
            _ => {}
        }
    }

    output
}
