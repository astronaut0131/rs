// add extern crates here, or in your lib.rs
extern crate ordered_float;
extern crate thrift;
extern crate try_from;

// generated Rust module


use std::net::TcpStream;
use std::thread;

use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
use thrift::protocol::{TInputProtocol, TOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport};
use thrift::transport::{TIoChannel, TTcpChannel};

use crate::sync_thrift_test::tutorial::{CalculatorSyncClient, TCalculatorSyncClient};

use self::thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use self::thrift::transport::{TBufferedReadTransport, TBufferedReadTransportFactory, TBufferedWriteTransport};

pub fn run(stream: TcpStream, loop_num: i32) -> thrift::Result<(Box<Vec<i64>>)> {
    //
    // build client
    //

    // println!("connect to server on 127.0.0.1:9090");

    let channel = TTcpChannel::with_stream(stream);

    let mut time_array = Vec::with_capacity(loop_num as usize);

    let (i_chan, o_chan) = channel.split()?;

    let i_prot = TBinaryInputProtocol::new(
        TBufferedReadTransport::new(i_chan), true,
    );
    let o_prot = TBinaryOutputProtocol::new(
        TBufferedWriteTransport::new(o_chan), true,
    );

    let mut client = CalculatorSyncClient::new(i_prot, o_prot);
    for i in 0..loop_num {
        let before = time::Instant::now();
        client.ping()?;
        let end = time::Instant::now();

        time_array.push((end - before).num_nanoseconds().unwrap());
    }

    //
    // println!("final result {}", sum);
    // println!("Test pass, It's time to cheer!");

    // done!
    // println!("finish client");
    Ok((Box::new(time_array)))
}