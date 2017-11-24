use futures::{Future, Sink, Stream};

use value::Value;
use errors::ApplicationError;

pub trait Application {
    fn call(
        &self,
        interface: &str,
        method: &str,
        args: &[Box<Value>],
    ) -> Box<Future<Item = &[Box<Value>], Error = ApplicationError>>;

    fn emit(&self, interface: &str, event: &str, args: &[Box<Value>]);
}

pub trait ClientApplication: Application {
    fn open_stream(
        &self,
        interface: &str,
    ) -> Stream<Item = &[u8], Error = ApplicationError>;
    fn open_sink(
        &self,
        interface: &str,
    ) -> Sink<SinkItem = &[u8], SinkError = ApplicationError>;
}
