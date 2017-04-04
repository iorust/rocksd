//! A simple client and server implementation fo a multiplexed, line-based
//! protocol

#![deny(warnings, missing_docs)]

extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_rpc;
extern crate tokio_service;
extern crate bytes;
extern crate protobuf;

mod server;
mod rocksdpb;