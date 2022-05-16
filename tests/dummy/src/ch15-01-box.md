## Using `Box<T>` to Point to Data on the Heap

The most straightforward smart pointer is a *box*, whose type is written
`Box<T>`. Boxes allow you to store data on the heap rather than the stack. What
remains on the stack is the pointer to the heap data. Refer to Chapter 4 to
review the difference between the stack and the heap.

#### More Information About the Cons List

A *cons list* is a data structure that comes from the Lisp programming language
and its dialects and is made up of nested pairs. Its name comes from the `cons`
function (short for “construct function”) in Lisp that constructs a new pair
from its two arguments. By calling `cons` on a pair consisting of a value and
another pair, we can construct cons lists made up of recursive pairs.

For example, here's a pseudocode representation of a cons list containing the
list 1, 2, 3 with each pair in parentheses:

```text
(1, (2, (3, Nil)))
```

Each item in a cons list contains two elements: the value of the current item
and the next item. The last item in the list contains only a value called `Nil`
without a next item. A cons list is produced by recursively calling the `cons`
function. The canonical name to denote the base case of the recursion is `Nil`.
Note that this is not the same as the “null” or “nil” concept in Chapter 6,
which is an invalid or absent value.

The cons list isn’t a commonly used data structure in Rust. Most of the time
when you have a list of items in Rust, `Vec<T>` is a better choice to use.
Other, more complex recursive data types *are* useful in various situations,
but by starting with the cons list in this chapter, we can explore how boxes
let us define a recursive data type without much distraction.

Listing 15-2 contains an enum definition for a cons list. Note that this code
won’t compile yet because the `List` type doesn’t have a known size, which
we’ll demonstrate.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

<span class="caption">Listing 15-2: The first attempt at defining an enum to
represent a cons list data structure of `i32` values</span>

> Note: We’re implementing a cons list that holds only `i32` values for the
> purposes of this example. We could have implemented it using generics, as we
> discussed in Chapter 10, to define a cons list type that could store values of
> any type.

Using the `List` type to store the list `1, 2, 3` would look like the code in
Listing 15-3:
