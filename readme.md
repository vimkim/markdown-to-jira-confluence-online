# Markdown To Jira Confluence Markup Converter

Super fast and efficient Markdown to Jira Confluence Markup Converter powered by **Rust** and **WebAssembly (Wasm)**. This tool allows users to seamlessly convert Markdown content into Confluence-compatible markup for enhanced documentation workflows.

![image](https://github.com/user-attachments/assets/ec5891f7-f51c-4243-aa12-a33bbe42e1b7)

---

## Features

- **Lightning Fast**: Built with Rust for optimal speed and performance.
- **Web Ready**: Powered by Wasm for live usage in web applications.
- **Markdown to Confluence**: Converts Markdown elements like:
  - Headings (`#`, `##`, etc.) into Confluence-style headings (`h1.`, `h2.`, etc.).
  - Lists (`-` or `1.`) into Confluence lists.
  - Code blocks into `{code}` blocks with language highlighting.
  - Inline code into `{code}` inline styles.
  - Emphasis and strong text into `_` and `*`.
- **Advanced Features**:
  - Handles HTML within Markdown for raw HTML passthrough.
  - Tracks list nesting levels for accurate rendering.
  - Customizable support for additional Markdown attributes (`id`, `class`, etc.).
- **Simple API**: Easy-to-use function for converting Markdown strings to Confluence markup.

---

## Installation and Setup

### For Web Applications

1. Add the Wasm module to your project.
2. Initialize the Wasm package and call the `convert_markdown_to_confluence` function.

Example (JavaScript):

```javascript
import init, {
  convert_markdown_to_confluence,
} from "./markdown_to_confluence_converter.js";

async function convert() {
  await init(); // Initialize the Wasm module
  const markdown = `
    # Welcome
    ## Subheading
    - Bullet item 1
    - Bullet item 2
    \`\`\`rust
    fn main() {
        println!("Hello, Confluence!");
    }
    \`\`\`
    `;
  const result = convert_markdown_to_confluence(markdown);
  console.log(result);
}
convert();
```

---

### For Rust Developers

Add this converter as part of your Rust application:

1. Import the module:

   ```rust
   use your_crate::convert_markdown_to_confluence;
   ```

2. Convert Markdown directly:

   ````rust
   fn main() {
       let markdown = "
       # Heading
       - Item 1
       - Item 2
       ```rust
       println!(\"Hello, Confluence!\");
       ```
       ";

       let confluence_markup = convert_markdown_to_confluence(markdown);
       println!("{}", confluence_markup);
   }
   ````

---

## How It Works

### Conversion Logic

1. Parses Markdown using `pulldown_cmark`.
2. Translates Markdown elements into Confluence-style syntax:
   - **Headings**: Maps Markdown headers (`#`, `##`, etc.) to `h1.`, `h2.`, etc.
   - **Lists**: Handles both ordered and unordered lists with proper nesting.
   - **Code Blocks**: Converts fenced or indented blocks to `{code}` sections.
   - **Inline Styles**: Supports emphasis (`_`), strong text (`*`), and inline code (`{code}`).

---

## Contributing

We welcome contributions to make this converter even better! Follow these steps to get started:

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/markdown-to-confluence.git
   cd markdown-to-confluence
   ```

2. Build the Wasm module:

   ```bash
   wasm-pack build --target web
   ```

3. Test the project locally with your Markdown examples.

Feel free to open issues or submit pull requests on [GitHub](https://github.com/your-username/markdown-to-confluence).

---

## License

This project is licensed under the [MIT License](LICENSE).

For questions or feedback, feel free to open an issue or contact us at [kimdhyungg@gmail.com](mailto:kimdhyungg@gmail.com).
