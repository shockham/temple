templé
========

Simple HTML templating in rust

```rust
let html_string = html("Hello",
    elem("div", "class=\"test\"",
        elem("span", "", String::from_str("content"))
    )
);
```
