Get all the book we can
```
git clone https://github.com/rust-lang/book books/the_book
git clone https://github.com/rust-lang/rust-by-example books/rust_by_example
git clone https://github.com/rust-lang/edition-guide books/rust_edition_guide
git clone https://github.com/rust-cli/book books/commandline
git clone https://github.com/rustwasm/book books/webasm
git clone https://github.com/rust-embedded/book books/embedded
git clone https://github.com/rust-lang/reference/ books/reference
git clone https://github.com/rust-lang/nomicon books/nomicon
git clone https://github.com/esp-rs/book books/esp
git clone https://github.com/rust-lang/cargo books/cargo
git clone https://github.com/rust-lang/rust books/rustdoc
```

Try to conver all of them into EPUBS

```
find -name "book.toml" -exec echo "{}" \; -exec bash -c 'mdbook-epub/target/debug/mdbook-epub -s `dirname {}`' \;
```

Check for correctly formed EPUBS and print only the errors

```
find -name "*.epub" -exec echo "{}" \; -exec java -jar epubcheck-4.2.6/epubcheck.jar "{}" -f \;
```

```
find -name "*.epub" -exec echo "{}" \; -exec rm "{}" \;
```
