<div align="center">
  <picture>
    <img
         src="resources/icons/maid.png"
         width="16%">
  </picture>

[Website](https://maidcode.me/) | [Download](https://crates.io/crates/maid-lang) | [Guidebook](https://maidcode.me/docs/getting-started/installation) | [Documentation](https://maidcode.me/docs/server-actions/getToken) | [Online Interpreter](https://maid-web-backend.fly.dev/)

_A very simple interpreted programming language for beginners._
</div>

## Code in Action

```
# import the math module
fetch std_math;

obj x = 0;

# let's go for a walk!
walk i = 0 through 10 {
    obj x = x + 1;

    if x == 5 {
        leave;
    }
}

# print the value of 'x'
serve(x);

# greet someone
func greet(name) {
    serve("Hello, " + name + "!");
}

greet("my Maid");

serve("Pi is equal to: " + tostring(math_pi));
serve("We have reached the end of our program. I hope you enjoyed!");
```

```
# import the math module
fetch std_math; # imports
serve(math_pi); # built in functions
obj x = 0; # object creation

# looping
walk i = 0 through 10 {
    serve("'i' is equal to: " + tostring(i));

    if i == 5 {
        leave;
    }
}

while 1 == 1 {
    serve("Inside a while loop");

    leave;
}

# function definitions
func greet(name) {
    serve("Hello my " + name + "!");

    give null;
}

greet("Maid");

```

## Features

- Built-in modules for math, strings, and more
- Easy-to-understand functions like `sweep()`, `stash()`, and `uhoh()`
- Package management with `kennels` and extensible with `fetch`
- Open source
- [Online Interpreter](https://maid-web-backend.fly.dev/) is live! Play with Maid in your browser!
## Package Installation

You can install **Maid** Programming Language globally using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) which comes pre-packaged when downloading Rust:

```bash
cargo install maid-lang

```
Once installed, run the maid command:
```
maid new hello-world
cd hello-world
maid run file.maid
```
or check out the quick setup instructions in the [Guide Book](https://maidcode.me/docs/getting-started/installation).

## Wanna Help Out?

Contributors are welcome! Also feel free to try my [Online Interpreter](https://maid-web-backend.fly.dev/) for Maid.

![Interpreter](https://github.com/xqyet/MaidCode/raw/main/resources/icons/interpreter.png)


## License

Maid is **free & open source.**
