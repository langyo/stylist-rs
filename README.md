# Stylist

[![Run Tests & Publishing](https://github.com/futursolo/stylist-rs/actions/workflows/everything.yml/badge.svg)](https://github.com/futursolo/stylist-rs/actions/workflows/everything.yml)
[![crates.io](https://img.shields.io/crates/v/stylist)](https://crates.io/crates/stylist)
[![download](https://img.shields.io/crates/d/stylist)](https://crates.io/crates/stylist)
[![docs.rs](https://docs.rs/stylist/badge.svg)](https://docs.rs/stylist/)

Stylist is a CSS-in-Rust styling solution for WebAssembly Applications.

This is a fork of [css-in-rust](https://github.com/lukidoescode/css-in-rust).

## Install

Add the following to your `Cargo.toml` if you want to use it in a browser environment:

```toml
stylist = "0.13"
```

Maybe you want to use `yew` integration:

```toml
stylist = { version = "0.13", features = ["yew_integration"]}
```

If you want to render styles on the server side, use `ssr` and `hydration` features:

```toml
stylist = { version = "0.13", features = ["yew_integration", "ssr", "hydration"]}
```

If you want to run inside a non-web WASM environment, such as [Cloudflare Worker](https://workers.cloudflare.com), use `not_browser_env` feature:

```toml
stylist = { version = "0.13", features = ["not_browser_env"]}
```

## Usage

For detailed usage, please see
[documentation](https://docs.rs/stylist/).

### Yew Integration

To style your component, you can use `styled_component` attribute with `css!`
macro.

```rust
use yew::prelude::*;
use stylist::yew::styled_component;

#[styled_component]
fn MyStyledComponent() -> Html {
    html! {<div class={css!("color: red;")}>{"Hello World!"}</div>}
}
```

### Standalone

To create a stylesheet, you can use `style!`:

```rust
use stylist::style;

let style = style!(
   // A CSS string literal
   r#"
       background-color: red;

       .nested {
           background-color: blue;
           width: 100px
       }
   "#
).expect("Failed to mount style");

// stylist-uSu9NZZu
println!("{}", style.get_class_name());
```

### Runtime Style

If you want to parse a string into a style at runtime, you can use `Style::new`:

```rust
use stylist::Style;

let style_str = r#"
    background-color: red;

    .nested {
        background-color: blue;
        width: 100px
    }
"#;

let style = Style::new(style_str).expect("Failed to create style");

// stylist-uSu9NZZu
println!("{}", style.get_class_name());
```

### Theming

There's theming example using
[Yew Context API](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-context).
