
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

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

fn does_nothing_desugared() -> impl Future<Output=()> {
    DoesNothingFuture {}
}

async fn read_file(file: &mut File) -> String {
    let mut v = Vec::new();
    file.read_to_end(&mut v).await.unwrap();
    String::from_utf8(v).unwrap()
}

enum ReadFileState {
    State0,
    State1(Pin<Box<dyn Future<Output=tokio::io::Result<usize>>>>),
}

struct ReadFileFuture<'a> {
    file: &'a mut File,
    v: Vec<u8>,
    state: ReadFileState,
    _pin: PhantomPinned,
}


impl<'a> Future for ReadFileFuture<'a> {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: There are no references into state
        let mut state = unsafe { &mut self.get_unchecked_mut().state };
        match self.state {
            ReadFileState::State0 => {
                let s = self;
                let file: &'a mut File = s.file;
                let fut = file.read_to_end(&mut self.v);
                let wrapped = Box::pin(fut);
                *state = ReadFileState::State1(wrapped);
                Poll::Pending
            },
            ReadFileState::State1(_) => {
                Poll::Pending
            },
        }
    }
}

fn read_file_desugared(file: &mut File) -> impl Future<Output=String> + '_ {
    ReadFileFuture {
        file,
        v: Vec::new(),
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
    println!("{}", read_file_desugared(&mut file).await);
}
