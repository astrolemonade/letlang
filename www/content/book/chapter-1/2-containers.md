---
title: 1.2. Containers
description: Description of Letlang's builtin container types
prev: /book/chapter-1/1-primitive-types
next: /book/chapter-1/3-sum-types
---

# Tuples

A tuple is a collection of values. There is no `tuple` class, instead, each
collection of types is the specific tuple class:

```letlang
(@ok, "hello") is (atom, string);  # true
(0, 0) is (number, number);        # true
(0.1, 0.2) is (int, int);          # false
```

Tuples are implemented using a Rust boxed array: `Box<[Value]>`.

# Lists

A list is a collection of values of the same type. There is no `list` class,
instead there is the `list<T>` generic class:

```letlang
[1, 2, 3] is list<int>;       # true
[1, 2, 3.5] is list<number>;  # true
[1, 2, 3.5] is list<int>;     # false
```

Lists are implemented using a Rust vector of values: `Vec<Value>`.

# Structures

A structure is a collection of named values. There is no `struct` class, instead
each structured type is the specific structure class:

```letlang
{x: 0, y: 0} is {x: number, y: number};   # true
```

Structures are implemented using a Rust hashmap:
`HashMap<String, Value>`.

# Implementation detail

Every **Letlang** value is represented by the following Rust enum:

```rust
struct Atom(usize);

enum Value {
  Boolean(bool),
  Number(f64),
  String(String),
  Atom(Atom),

  Tuple(Box<[Value]>),
  List(Vec<Value>),
  Struct(HashMap<String, Value>),

  // function types, see next chapter
}
```