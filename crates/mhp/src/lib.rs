extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use bytes::BytesMut;
use futures::{future, Future};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;

pub struct MhpCodec;

impl Decoder for MhpCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if let Some(lf_index) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(lf_index);
            buf.split_to(1);

            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_owned())),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "invalid UTF-8 sequence",
                )),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for MhpCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

pub struct MhpProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for MhpProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, MhpCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(MhpCodec))
    }
}

pub struct EchoService;

impl Service for EchoService {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        Box::new(future::ok(req))
    }
}
