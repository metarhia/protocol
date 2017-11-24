use futures::{Future, Sink, Stream};

use application::*;
use value::Value;
use errors::ApplicationError;

pub trait Transport {}

pub struct Connection {
    transport: Box<Transport>,
    application: Box<Application>,
    remote_api: Box<ClientApplication>,
    next_chunk_id: i32,
    chunk_id_step: i32,
}

impl Connection {
    pub fn new(
        transport: Box<Transport>,
        application: Box<Application>,
        remote_api: Box<ClientApplication>,
    ) -> Self {
        !unimplemented!();
    }

    pub fn handshake(
        &self,
        application: &str,
        credentials: Box<Value>,
    ) -> Box<Future<Item = Box<Value>, Error = ApplicationError>> {
        unimplemented!();
    }

    pub fn inspect(
        &self,
        interface: &str,
    ) -> Box<Future<Item = Box<Value>, Error = ApplicationError>> {
        unimplemented!();
    }

    pub fn call(
        &self,
        interface: &str,
        method: &str,
        args: &[Box<Value>],
    ) -> Box<Future<Item = &[Box<Value>], Error = ApplicationError>> {
        unimplemented!();
    }

    pub fn emit(&self, interface: &str, event: &str, args: &[Box<Value>]) {
        unimplemented!();
    }

    pub fn open_stream(
        &self,
        interface: &str,
    ) -> Box<Stream<Item = &[u8], Error = ApplicationError>> {
        unimplemented!();
    }
    pub fn open_sink(
        &self,
        interface: &str,
    ) -> Box<Sink<SinkItem = &[u8], SinkError = ApplicationError>> {
        unimplemented!();
    }

    pub fn start_heartbeat() {
        !unimplemented!();
    }

    pub fn stop_heartbeat() {
        !unimplemented!();
    }

    pub fn close() {
        !unimplemented!();
    }

    fn callback(&self, args: &[Box<Value])> {
        !unimplemented!();
    }

    fn ping(&self) {
        unimplemented!();
    }

    fn pong(&self) {
        unimplemented!();
    }
}
