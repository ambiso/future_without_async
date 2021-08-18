use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn does_nothing() {}

struct DoesNothingFuture;

impl Future for DoesNothingFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

fn does_nothing_desugared() -> impl Future<Output = ()> {
    DoesNothingFuture {}
}

async fn read_file(file: &mut File) -> String {
    let mut v = Vec::new();
    file.read_to_end(&mut v).await.unwrap();
    String::from_utf8(v).unwrap()
}

struct ReadFileFuture<'a> {
    file: &'a mut File,
    v: Option<Vec<u8>>,
    state: ReadFileState<'a>,
    _pin: PhantomPinned,
}

enum ReadFileState<'a> {
    State0,
    State1(Pin<Box<dyn Future<Output = tokio::io::Result<usize>> + 'a>>),
}

impl<'a> Future for ReadFileFuture<'a> {
    type Output = String;

    fn poll<'b>(mut self: Pin<&'b mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = unsafe { self.as_mut().get_unchecked_mut() };
        loop {
            match s.state {
                ReadFileState::State0 => {
                    let fut = s.file.read_to_end(s.v.as_mut().unwrap());
                    let wrapped = Box::pin(fut);
                    let new_state = unsafe {
                        std::mem::transmute::<_, ReadFileState<'a>>(ReadFileState::State1(wrapped))
                    };
                    s.state = new_state;
                }
                ReadFileState::State1(ref mut fut) => {
                    let r = fut.as_mut().poll(cx);
                    if r.is_pending() {
                        return Poll::Pending;
                    }
                    let v = s.v.take().unwrap();
                    return Poll::Ready(String::from_utf8(v).unwrap());
                }
            }
        }
    }
}

fn read_file_desugared(file: &mut File) -> impl Future<Output = String> + '_ {
    ReadFileFuture {
        file,
        v: Some(Vec::new()),
        state: ReadFileState::State0,
        _pin: PhantomPinned {},
    }
}

#[tokio::main]
async fn main() {
    does_nothing().await;
    does_nothing_desugared().await;

    let mut file = File::open("test").await.unwrap();
    println!("{}", read_file(&mut file).await);
    let mut file = File::open("test").await.unwrap();
    println!("{}", read_file_desugared(&mut file).await);
}
