# Formatter Settings

Mago’s formatter can be configured to suit your project’s needs. This page covers the available settings and how to set them up.

The default settings are designed to work well for most projects, and adhere to the [PER Coding Style 2.0](https://www.php-fig.org/per/coding-style/) coding standard,
but you can customize them to match your preferred coding style.

## Formatting Settings

### `print_width`

Specifies the maximum line length before the formatter wraps lines. This helps enforce a consistent code width.

- Default: `120`
- Type: `integer`
- Example:

  ```toml
  print_width = 100
  ```

### `tab_width`

Sets the number of spaces to use for each tab character.

- Default: `4`
- Type: `integer`
- Example:

  ```toml
  tab_width = 2
  ```

### `use_tabs`

Controls whether the formatter uses tabs or spaces for indentation.

- Default: `false`
- Type: `boolean`
- Example:

  ```toml
  use_tabs = true
  ```

### `end_of_line`

Specifies the line ending style to use.

- Default: `lf`
- Type: `enum { "auto", "lf", "crlf", "cr" }`
- Example:

  ```toml
  end_of_line = "crlf"
  ```

### `single_quote`

Determines whether single quotes are preferred over double quotes for strings.

- Default: `true`
- Type: `boolean`
- Example:

  ```toml
  single_quote = false
  ```

### `trailing_comma`

Adds trailing commas to multi-line syntactic structures, such as arrays and parameter lists.

- Default: `true`
- Type: `boolean`
- Example:

  ```toml
  trailing_comma = false
  ```

### `space_around_declare_equals`

Controls whether spaces are added around the `=` sign in declare statements.

- Default: `true`
- Type: `boolean`
- Example:

  ```toml
  space_around_declare_equals = false
  ```

### `keyword_case`

Specifies the case to use for keywords.

- Default: `"lowercase"`
- Type: `enum { "lowercase", "uppercase" }`
- Example:

  ```toml
  keyword_case = "uppercase"
  ```

### `blank_line_after_open_tag`

Adds a blank line after the opening tag of a PHP file.

- Default: `true`
- Type: `boolean`
- Example:

  ```toml
  blank_line_after_open_tag = false
  ```

### `control_space_parens`

Controls whether spaces are added inside parentheses in control structures.

- Default: `false`
- Type: `boolean`
- Example:

  ```toml
  control_space_parens = true
  ```

### `closure_brace_style`

Specifies the style to use for braces in closures.

- Default: `"same_line"`
- Type: `enum { "same_line", "next_line" }`
- Example:

  ```toml
  closure_brace_style = "next_line"
  ```

### `function_brace_style`

Specifies the style to use for braces in functions.

- Default: `"next_line"`
- Type: `enum { "same_line", "next_line" }`
- Example:

  ```toml
  function_brace_style = "same_line"
  ```

### `method_brace_style`

Specifies the style to use for braces in methods.

- Default: `"next_line"`
- Type: `enum { "same_line", "next_line" }`
- Example:

  ```toml
  method_brace_style = "same_line"
  ```

### `classlike_brace_style`

Specifies the style to use for braces in class-like structures (classes, interfaces, traits, enums, and anonymous classes).

- Default: `"next_line"`
- Type: `enum { "same_line", "next_line" }`
- Example:

  ```toml
  classlike_brace_style = "same_line"
  ```

### `control_brace_style`

Specifies the style to use for braces in control structures.

- Default: `"same_line"`
- Type: `enum { "same_line", "next_line" }`
- Example:

  ```toml
  control_brace_style = "next_line"
  ```

### `space_before_closure_params`

Controls whether a space is added before the parameter list in closures.

- Default: `true`
- Type: `boolean`
- Example:

  ```toml
  space_before_closure_params = false
  ```

### `space_after_closure_use`

Controls whether a space is added after the `use` keyword in closures.

- Default: `true`
- Type: `boolean`
- Example:

  ```toml
  space_after_closure_use = false
  ```

### `space_before_arrow_function_params`

Controls whether a space is added before the parameter list in arrow functions.

- Default: `true`
- Type: `boolean`
- Example:

  ```toml
  space_before_arrow_function_params = false
  ```

### `static_before_visibility`

Controls whether the `static` keyword is placed before the visibility keyword in class-like members.

- Default: `false`
- Type: `boolean`
- Example:

  ```toml
  static_before_visibility = true
  ```

> This setting also affects the order of the `readonly` keyword, if present.

### `null_type_hint`

Specifies the type hint style to use for nullable types.

- Default: `"null_pipe"`
- Type: `enum { "question", "null_pipe" }`
- Example:

  ```toml
  null_type_hint = "question"
  ```

### `type_spacing`

Controls the number of spaces to add around types in a compound type (e.g., `int | string`, `int & null`, etc.).

- Default: `0`
- Type: `integer`
- Example:

  ```toml
  type_spacing = 1
  ```

### `method_chain_break_threshold`

Specifies the maximum number of method calls allowed before breaking the chain.

- Default: `4`
- Type: `integer`
- Example:

  ```toml
  method_chain_break_threshold = 2
  ```

### `break_promoted_properties_list`

Whether to break a parameter list into multiple lines if it contains one or more promoted property even
if it fits into a single line.

- Default: `true`
- Type: `boolean`
- Example:

  ```toml
  break_promoted_properties_list = false
  ```

### `space_concatenation`

Controls whether spaces are added around the concatenation operator (`.`).

- Default: `true`
- Type: `boolean`
- Example:

  ```toml
  space_concatenation = false
  ```

### `method_chain_breaking_style`

Specifies the style to use for breaking method chains.

- Default: `"next_line"`
- Type: `enum { "next_line", "same_line" }`
- Example:

  ```toml
  method_chain_breaking_style = "same_line"
  ```

### `line_before_binary_operator`

Controls whether a line break is added before or after binary operators when breaking lines.

- Default: `false`
- Type: `boolean`
- Example:

  ```toml
  line_before_binary_operator = true
  ```

> This setting will always be false if the rhs of the binary operator has a leading comment.
