## [`AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/io/trait.AsyncWrite.html)

When deriving for enum like the following:

```rust
#[enum_derive(AsyncWrite)]
enum Enum<A, B> {
    A(A),
    B(B),
}
```

Code like this will be generated:

```rust
enum Enum<A, B> {
    A(A),
    B(B),
}

#[allow(unsafe_code)]
impl<A, B> ::futures::io::AsyncWrite for Enum<A, B>
where
    A: ::futures::io::AsyncWrite,
    B: ::futures::io::AsyncWrite,
{
    #[inline]
    fn poll_write(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
        buf: &[u8],
    ) -> ::core::task::Poll<::std::io::Result<usize>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::io::AsyncWrite::poll_write(::core::pin::Pin::new_unchecked(x), cx, buf),
                Enum::B(x) => ::futures::io::AsyncWrite::poll_write(::core::pin::Pin::new_unchecked(x), cx, buf),
            }
        }
    }

    #[inline]
    fn poll_write_vectored(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
        bufs: &[::std::io::IoSlice<'_>],
    ) -> ::core::task::Poll<::std::io::Result<usize>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::io::AsyncWrite::poll_write_vectored(::core::pin::Pin::new_unchecked(x), cx, bufs),
                Enum::B(x) => ::futures::io::AsyncWrite::poll_write_vectored(::core::pin::Pin::new_unchecked(x), cx, bufs),
            }
        }
    }

    #[inline]
    fn poll_flush(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::std::io::Result<()>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::io::AsyncWrite::poll_flush(::core::pin::Pin::new_unchecked(x), cx),
                Enum::B(x) => ::futures::io::AsyncWrite::poll_flush(::core::pin::Pin::new_unchecked(x), cx),
            }
        }
    }

    #[inline]
    fn poll_close(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::std::io::Result<()>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::io::AsyncWrite::poll_close(::core::pin::Pin::new_unchecked(x), cx),
                Enum::B(x) => ::futures::io::AsyncWrite::poll_close(::core::pin::Pin::new_unchecked(x), cx),
            }
        }
    }
}
```
