# Documentation

**IMPORTANT: Elements marked with [*] are not yet part of the language, and only planned features and ideas**

__nen__ is a functional programming language written in Rust. It ships with two programs:

- _nenc_: A compiler to compile __nen__ code into bytecode.
- _nen_[*]: An interpreter/VM for __nen__ bytecode.

## Syntax

All statements in __nen__ are terminated with a semicolon (`;`), and blocks are enclosed in curly brackets (`{}`).

Comments begin with an octothorpe (`#`) and continue to the end of the line [*].

## Functions

Functions in __nen__ are defined with the `func` keyword, like below:

```nen
func example() {
	# body
}
```

There are a number of *attributes* a function can take, and they are:

- impure: Specifies that the function has [side-effects](https://en.wikipedia.org/wiki/Side_effect_(computer_science)), such as printing to the console or opening a file.
- async [*]: Specifies that the function contains logic that runs asynchronously to the main thread, such as making a HTTP request and waiting for the response.

Arguments are specified in the format `name: type`, and separated by commas, like so:

```nen
impure func print(input: string) {
	# Body
}
```

A return value can be specified after the function arguments with `: type` as such:

```nen
func double(x: int): int {
	return x * 2;
}
```
