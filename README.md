# mediawiki-parser
This project aims to develop a parser for a subset of mediawiki markdown on the basis of Parsing Expression Grammars. 
It currently features a generated parser and test generation from a specification document. A simple binary to read from a file and write yaml to stdout is provided.

## Disclaimer

The goal of mediawiki-parser is *not* full compatibility with MediaWiki and all of it's quirks. It is intended to be used if rejecting exotic or malformed input is fine. 
The markup supported is currently largely oriented towards the need of a specific MediaWiki Project and will likely not change drastically without external contributions. 

If you want to parse any MediaWiki with all its weirdness, take a look at [Parse Wiki Text](https://github.com/portstrom/parse_wiki_text) instead.

## Currently supported MediaWiki:

* Text formatting: `''italic'', '''bold''', <math>\LaTex</math>, <code></code>, ...`
* Paragraphs
* Heading hierarchies
* Lists
* Internal references (files) `[[File.ext|option|caption]]`
* External references `[https://example.com/ example]`
* Tables
* Generic templates `{{name|anon_arg|arg=value}}`
* Galleries
* Generic html tags and comments `<thing>content</thing>`

## Known Limitations

This project has some known limitations, which might or might not be lifted in the future. 
Part of this comes from treating WikiText as a context-free formal language, which is not entrierly true.

* `{,},[,]`  cannot be used in plain text, as they normally indicate special syntax. However, using them in math or `<nowiki>` is fine.
* Indentation is currently not parsed as `pre`.
* Templates are only pared on a syntactical level, they have no effects on their content whatsoever.


## Example

Parsing will result in either a syntax tree with position information (mostly omitted here for conciseness):

Input:
``` markdown
this is some ''formatted'' [https://example.com example] text.
```
Output (as pseudo-YAML):
``` yaml
---
type: document
position: ...
content:
  - type: paragraph
    position: ...
    content:
      - type: text
        position: ...
        text: "this is some "
      - type: formatted
        position: ...
        markup: italic
        content:
          - type: text
            position:
              start:
                offset: 15
                line: 1
                col: 16
              end:
                offset: 24
                line: 1
                col: 25
            text: formatted
      - type: text
        position: ...
        text: " "
      - type: externalreference
        position: ...
        target: "https://example.com"
        caption:
          - type: text
            position: ...
            text: example
      - type: text
        position: ...
        text: " text."
```

Or a syntax error (here is a pretty representation):
```
ERROR in line 1 at column 57: Could not continue to parse, expected one of: ''', [, <!--, '', [[, EOF, "\n", {{, [ 	], opening html tag, <, normal text
1 | this is some ''formatted'' [https://example.com example]] text.
2 |
``` 

## API

The library provides a straight forward `parse()` function:

```rust
let input = "Hello World";
let result = mediawiki_parser::parse(&input)
    .expect(\"Parsing of the input for {} failed!\");
println!(\"{{}}\", &serde_yaml::to_string(&result).unwrap());
```

The result is a custom abstract syntax tree (AST). See the documentation for details.

