/* dummy implementation for tcp time out
 * since timeout for tcp in std doesn't implemented for linux yet
 * and the server of hangman is really poor
 */
use std::io;
use std::net::TcpStream;
use hyper::net::NetworkConnector;
use hyper::net::NetworkStream;
use std::fmt;
use std::net::SocketAddr;
use std::io::Read;
use std::io::Write;
use std::mem::transmute;
use std::mem;
use libc::c_void;
use libc::c_int;
use libc::socklen_t;
use libc::c_longlong;
use std::num::SignedInt;
use std::num::Int;
use openssl::ssl::{SslContext,SslStream};
use openssl::ssl::SslMethod::*;
use openssl::ssl::Ssl;
use openssl::ssl::error::{SslError, StreamError, OpenSslErrors, SslSessionClosed};

/// A connector that will produce HttpStreams.
pub struct HttpConnector(pub Option<ContextVerifier>, pub usize);

/// A method that can set verification methods on an SSL context
pub type ContextVerifier = Box<FnMut(&mut SslContext) -> () + Send>;

impl NetworkConnector for HttpConnector {
    type Stream = HttpStream;

    fn connect(&mut self, host: &str, port: u16, scheme: &str) -> io::Result<HttpStream> {
        let addr = &(host, port);
        match scheme {
            "http" => {
                debug!("http scheme");
                let s = try!(TcpStream::connect(addr));
                let (sock,dtor): (c_int, c_int) = unsafe {transmute(s)};
                let timeout: (c_longlong, c_longlong) = (self.1 as c_longlong, 0);
                try!(setsockopt(&sock, ::libc::SOL_SOCKET, ::libc::SO_SNDTIMEO, &timeout));
                try!(setsockopt(&sock, ::libc::SOL_SOCKET, ::libc::SO_RCVTIMEO, &timeout));
                let s = unsafe {transmute((sock,dtor))};

                Ok(HttpStream::Http(CloneTcpStream(s)))
            },
            "https" => {
                debug!("https scheme");
                let s = try!(TcpStream::connect(addr));
                let (sock,dtor): (c_int, c_int) = unsafe {transmute(s)};
                let timeout: (c_longlong, c_longlong) = (self.1 as c_longlong, 0);
                try!(setsockopt(&sock, ::libc::SOL_SOCKET, ::libc::SO_SNDTIMEO, timeout));
                try!(setsockopt(&sock, ::libc::SOL_SOCKET, ::libc::SO_RCVTIMEO, timeout));
                let s = unsafe {transmute((sock, dtor))};

                let stream = CloneTcpStream(s);
                let mut context = try!(SslContext::new(Sslv23).map_err(lift_ssl_error));
                if let Some(ref mut verifier) = self.0 {
                    verifier(&mut context);
                }
                let ssl = try!(Ssl::new(&context).map_err(lift_ssl_error));
                try!(ssl.set_hostname(host).map_err(lift_ssl_error));
                let stream = try!(SslStream::new(&context, stream).map_err(lift_ssl_error));
                Ok(HttpStream::Https(stream))
            },
            _ => {
                Err(io::Error::new(io::ErrorKind::InvalidInput,
                                "Invalid scheme for Http"))
            }
        }
    }
}

#[doc(hidden)]
pub struct CloneTcpStream(TcpStream);

impl Clone for CloneTcpStream{
    #[inline]
    fn clone(&self) -> CloneTcpStream {
        CloneTcpStream(self.0.try_clone().unwrap())
    }
}

impl Read for CloneTcpStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl Write for CloneTcpStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }
    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

/// A wrapper around a TcpStream.
#[derive(Clone)]
pub enum HttpStream {
    /// A stream over the HTTP protocol.
    Http(CloneTcpStream),
    /// A stream over the HTTP protocol, protected by SSL.
    Https(SslStream<CloneTcpStream>),
}

impl fmt::Debug for HttpStream {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      HttpStream::Http(_) => write!(fmt, "Http HttpStream"),
      HttpStream::Https(_) => write!(fmt, "Https HttpStream"),
    }
  }
}

impl Read for HttpStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            HttpStream::Http(ref mut inner) => inner.read(buf),
            HttpStream::Https(ref mut inner) => inner.read(buf)
        }
    }
}

impl Write for HttpStream {
    #[inline]
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        match *self {
            HttpStream::Http(ref mut inner) => inner.write(msg),
            HttpStream::Https(ref mut inner) => inner.write(msg)
        }
    }
    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        match *self {
            HttpStream::Http(ref mut inner) => inner.flush(),
            HttpStream::Https(ref mut inner) => inner.flush(),
        }
    }
}

impl NetworkStream for HttpStream {
    fn peer_addr(&mut self) -> io::Result<SocketAddr> {
        match *self {
            HttpStream::Http(ref mut inner) => inner.0.peer_addr(),
            HttpStream::Https(ref mut inner) => inner.get_mut().0.peer_addr()
        }
    }
}

fn lift_ssl_error(ssl: SslError) -> io::Error {
    debug!("lift_ssl_error: {:?}", ssl);
    match ssl {
        StreamError(err) => err,
        SslSessionClosed => io::Error::new(io::ErrorKind::ConnectionAborted,
                                         "SSL Connection Closed"),
        e@OpenSslErrors(..) => io::Error::new(io::ErrorKind::Other, e)
    }
}

fn setsockopt<T>(sock: &c_int, opt: c_int, val: c_int,
                     payload: T) -> io::Result<()> {
    unsafe {
        let payload = &payload as *const T as *const c_void;
        try!(cvt(::libc::setsockopt(*sock, opt, val, payload,
                                  mem::size_of::<T>() as socklen_t)));
        Ok(())
    }
}
pub fn cvt<T: SignedInt>(t: T) -> io::Result<T> {
    let one: T = Int::one();
    if t == -one {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}
