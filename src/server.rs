use std::io;
use futures::{future, Future};
use tokio_service::{Service, NewService};
use protobuf::core::{Message, parse_from_bytes};

use super::rocksdpb;

/// RocksD
struct RocksD;

impl Service for RocksD {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let req = parse_from_bytes::<rocksdpb::Request>(req.as_slice());
        if let Err(err) = req {
            return future::err(io::Error::new(io::ErrorKind::Other, err)).boxed();
        }

        let req = req.unwrap();
        let res_todo = rocksdpb::Response::new();
        let res: rocksdpb::Response = match req.get_field_type() {
            rocksdpb::MessageType::CmdQuit => res_todo,
            rocksdpb::MessageType::CmdGet => res_todo,
            rocksdpb::MessageType::CmdSet => res_todo,
            rocksdpb::MessageType::CmdPing => {
                let req = req.get_cmd_ping_req();

                let mut msg = rocksdpb::CmdPingResponse::new();
                msg.set_message(req.get_message().to_string());
                let mut res = rocksdpb::Response::new();
                res.set_field_type(rocksdpb::MessageType::CmdPing);
                res.set_cmd_ping_res(msg);
                res
            }
        };

        match res.write_to_bytes() {
            Ok(val) => future::ok(val).boxed(),
            Err(err) => future::err(io::Error::new(io::ErrorKind::Other, err)).boxed(),
        }
    }
}

impl NewService for RocksD {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Instance = RocksD;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(RocksD {})
    }
}
