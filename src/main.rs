extern crate openssl;

use std::net::TcpStream;
use openssl::ssl::{Ssl, SslContext, SslMethod, SslStream};

fn main() {
    let context = SslContext::new(SslMethod::Tlsv1).unwrap();
    let ssl = Ssl::new(&context).unwrap();
    let inner = TcpStream::connect("example.com:443").unwrap();
    let stream1 = SslStream::connect(ssl, inner).unwrap();
    let _stream2 = stream1.try_clone().unwrap();
}
