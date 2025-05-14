# rotchess-rust

I've been looking into refactoring the code base for
[rotating chess](https://github.com/wade-cheng/rotating-chess), and thought I
might as well try out Rust with its better static type checking.

As of writing, `macroquad` is doing the brunt of the work as the game library,
and `statig` is helping with hierarchical state machines (for screens and,
later, ui. I think.).
