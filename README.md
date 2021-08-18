# Desugared `async fn`s

Here I implemented two simple futures without using `async fn`.

## A simple async function

We first look at the simplest possible example:

```rust
async fn does_nothing() {}
```

An `async` function boils down to a function returning some type that implements the [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html#) trait:


```rust
fn does_nothing_desugared() -> impl Future<Output=()> {
    DoesNothingFuture {}
}
```

The `Future` trait looks like this:

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

It offers a poll function that an async runtime can call to make progress on the future.


## A more complex example

Now lets try desugaring a more complex async function.

```rust
async fn read_file(file: &mut File) -> String {
    let mut v = Vec::new();
    file.read_to_end(&mut v).await.unwrap();
    String::from_utf8(v).unwrap()
}
```

`read_file` is more complex in that it has an argument and awaits another future.

The effort fairly involved since you cannot name the lifetime of another struct member.
Therefore, `unsafe` is needed to circumvent the restrictions of `!Unpin`.

## Thanks

- [Jon Gjengset: The What and How of Futures and async/await in Rust](https://www.youtube.com/watch?v=9_3krAQtD2k)
- [Jon Gjengset: The Why, What, and How of Pinning in Rust](https://www.youtube.com/watch?v=DkMwYxfSYNQ)
- [Yandros](https://users.rust-lang.org/t/desugaring-async-fn/63698/2)