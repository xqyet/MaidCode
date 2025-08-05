<div align="center">
  <picture>
    <img
         src="resources/icons/maid.png"
         width="16%">
  </picture>

[Website](https://sites.google.com/view/george-lang/home?authuser=0) | [Guidebook]( ) | [Documentation]( ) | [Online Interpreter]( )

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

## Installation

You can download the Maid Programming Language [here]( ), or check out the quick setup instructions in the [Guide Book]( ).

## Wanna Help Out?

Contributors and testers are welcome! Check out [CONTRIBUTING.md](./CONTRIBUTING.md) when it's added (soon!). Also feel free to try my [Online Interpreter]( ) for Maid (updating soon!)

## License

Maid is **free, open source, and always will be**.
