# Desugared `async fn`s

Here I implemented two simple futures without using `async fn`.

First a very simple one:

```rust
async fn does_nothing() {}
```

Then a slightly more complex one:

```rust
async fn read_file(file: &mut File) -> String {
    let mut v = Vec::new();
    file.read_to_end(&mut v).await.unwrap();
    String::from_utf8(v).unwrap()
}
```

The effort fairly involved since you cannot name the lifetime of another struct member.
Therefore, `unsafe` is needed to circumvent the restrictions of `!Unpin`.

## Thanks

- [Jon Gjengset: The What and How of Futures and async/await in Rust](https://www.youtube.com/watch?v=9_3krAQtD2k)
- [Jon Gjengset: The Why, What, and How of Pinning in Rust](https://www.youtube.com/watch?v=DkMwYxfSYNQ)
- [Yandros](https://users.rust-lang.org/t/desugaring-async-fn/63698/2)