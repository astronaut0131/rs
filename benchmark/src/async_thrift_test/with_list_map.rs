// Autogenerated by Thrift Compiler ()
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate async_thrift;

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{From, TryFrom};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use async_thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use async_thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TAsyncInputProtocol, TAsyncOutputProtocol, TSetIdentifier, TStructIdentifier, TType};
use async_thrift::protocol::field_id;
use async_thrift::protocol::verify_expected_message_type;
use async_thrift::protocol::verify_expected_sequence_number;
use async_thrift::protocol::verify_expected_service_call;
use async_thrift::protocol::verify_required_field_exists;
use async_trait::async_trait;
use self::async_thrift::server::TAsyncProcessor;

//
// ListMapTest service client
//

#[async_trait]
pub trait TListMapTestSyncClient {
  async fn sum_up(&mut self, input: Vec<i32>) -> async_thrift::Result<i32>;
  async fn find_value(&mut self, input: BTreeMap<i32, i32>) -> async_thrift::Result<i32>;
}

pub trait TListMapTestSyncClientMarker {}

pub struct ListMapTestSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {
  _i_prot: IP,
  _o_prot: OP,
  _sequence_number: i32,
}

impl <IP, OP> ListMapTestSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {
  pub fn new(input_protocol: IP, output_protocol: OP) -> ListMapTestSyncClient<IP, OP> {
    ListMapTestSyncClient { _i_prot: input_protocol, _o_prot: output_protocol, _sequence_number: 0 }
  }
}

impl <IP, OP> TThriftClient for ListMapTestSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {
  fn i_prot_mut(&mut self) -> &mut (dyn TAsyncInputProtocol + Send) { &mut self._i_prot }
  fn o_prot_mut(&mut self) -> &mut (dyn TAsyncOutputProtocol + Send) { &mut self._o_prot }
  fn sequence_number(&self) -> i32 { self._sequence_number }
  fn increment_sequence_number(&mut self) -> i32 { self._sequence_number += 1; self._sequence_number }
}

impl <IP, OP> TListMapTestSyncClientMarker for ListMapTestSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {}

#[async_trait]
impl <C: TThriftClient + TListMapTestSyncClientMarker+ Send> TListMapTestSyncClient for C {
  async fn sum_up(&mut self, input: Vec<i32>) -> async_thrift::Result<i32> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("sum_up", TMessageType::Call, self.sequence_number());
        let call_args = ListMapTestSumUpArgs { input: input };
        self.o_prot_mut().write_message_begin(&message_ident).await?;
        call_args.write_to_out_protocol(self.o_prot_mut()).await?;
        self.o_prot_mut().write_message_end().await?;
        self.o_prot_mut().flush().await
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin().await?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("sum_up", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = async_thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut()).await?;
        self.i_prot_mut().read_message_end().await?;
        return Err(async_thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = ListMapTestSumUpResult::read_from_in_protocol(self.i_prot_mut()).await?;
      self.i_prot_mut().read_message_end().await?;
      result.ok_or()
    }
  }
  async fn find_value(&mut self, input: BTreeMap<i32, i32>) -> async_thrift::Result<i32> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("find_value", TMessageType::Call, self.sequence_number());
        let call_args = ListMapTestFindValueArgs { input: input };
        self.o_prot_mut().write_message_begin(&message_ident).await?;
        call_args.write_to_out_protocol(self.o_prot_mut()).await?;
        self.o_prot_mut().write_message_end().await?;
        self.o_prot_mut().flush().await
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin().await?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("find_value", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = async_thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut()).await?;
        self.i_prot_mut().read_message_end().await?;
        return Err(async_thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = ListMapTestFindValueResult::read_from_in_protocol(self.i_prot_mut()).await?;
      self.i_prot_mut().read_message_end().await?;
      result.ok_or()
    }
  }
}

//
// ListMapTest service processor
//

#[async_trait]
pub trait ListMapTestSyncHandler {
  async fn handle_sum_up(&self, input: Vec<i32>) -> async_thrift::Result<i32>;
  async fn handle_find_value(&self, input: BTreeMap<i32, i32>) -> async_thrift::Result<i32>;
}

pub struct ListMapTestSyncProcessor<H: ListMapTestSyncHandler> {
  handler: H,
}

impl <H: ListMapTestSyncHandler> ListMapTestSyncProcessor<H> {
  pub fn new(handler: H) -> ListMapTestSyncProcessor<H> {
    ListMapTestSyncProcessor {
      handler,
    }
  }
  async fn process_sum_up(&self, incoming_sequence_number: i32, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    TListMapTestProcessFunctions::process_sum_up(&self.handler, incoming_sequence_number, i_prot, o_prot).await
  }
  async fn process_find_value(&self, incoming_sequence_number: i32, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    TListMapTestProcessFunctions::process_find_value(&self.handler, incoming_sequence_number, i_prot, o_prot).await
  }
}

pub struct TListMapTestProcessFunctions;

impl TListMapTestProcessFunctions {
  pub async fn process_sum_up<H: ListMapTestSyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let args = ListMapTestSumUpArgs::read_from_in_protocol(i_prot).await?;
    match handler.handle_sum_up(args.input).await {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("sum_up", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident).await?;
        let ret = ListMapTestSumUpResult { result_value: Some(handler_return) };
        ret.write_to_out_protocol(o_prot).await?;
        o_prot.write_message_end().await?;
        o_prot.flush().await
      },
      Err(e) => {
        match e {
          async_thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("sum_up", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident).await?;
            async_thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot).await?;
            o_prot.write_message_end().await?;
            o_prot.flush().await
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("sum_up", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident).await?;
            async_thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot).await?;
            o_prot.write_message_end().await?;
            o_prot.flush().await
          },
        }
      },
    }
  }
  pub async fn process_find_value<H: ListMapTestSyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let args = ListMapTestFindValueArgs::read_from_in_protocol(i_prot).await?;
    match handler.handle_find_value(args.input).await {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("find_value", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident).await?;
        let ret = ListMapTestFindValueResult { result_value: Some(handler_return) };
        ret.write_to_out_protocol(o_prot).await?;
        o_prot.write_message_end().await?;
        o_prot.flush().await
      },
      Err(e) => {
        match e {
          async_thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("find_value", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident).await?;
            async_thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot).await?;
            o_prot.write_message_end().await?;
            o_prot.flush().await
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("find_value", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident).await?;
            async_thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot).await?;
            o_prot.write_message_end().await?;
            o_prot.flush().await
          },
        }
      },
    }
  }
}

#[async_trait]
impl <H: ListMapTestSyncHandler + Send + Sync> TAsyncProcessor for ListMapTestSyncProcessor<H> {
  async fn process(&self, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let message_ident = i_prot.read_message_begin().await?;
    let res = match &*message_ident.name {
      "sum_up" => {
        self.process_sum_up(message_ident.sequence_number, i_prot, o_prot).await
      },
      "find_value" => {
        self.process_find_value(message_ident.sequence_number, i_prot, o_prot).await
      },
      method => {
        Err(
          async_thrift::Error::Application(
            ApplicationError::new(
              ApplicationErrorKind::UnknownMethod,
              format!("unknown method {}", method)
            )
          )
        )
      },
    };
    async_thrift::server::handle_process_result(&message_ident, res, o_prot).await
  }
}

//
// ListMapTestSumUpArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ListMapTestSumUpArgs {
  input: Vec<i32>,
}

impl ListMapTestSumUpArgs {
  async fn read_from_in_protocol(i_prot: &mut (dyn TAsyncInputProtocol + Send)) -> async_thrift::Result<ListMapTestSumUpArgs> {
    i_prot.read_struct_begin().await?;
    let mut f_1: Option<Vec<i32>> = None;
    loop {
      let field_ident = i_prot.read_field_begin().await?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin().await?;
          let mut val: Vec<i32> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_0 = i_prot.read_i32().await?;
            val.push(list_elem_0);
          }
          i_prot.read_list_end().await?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type).await?;
        },
      };
      i_prot.read_field_end().await?;
    }
    i_prot.read_struct_end().await?;
    verify_required_field_exists("ListMapTestSumUpArgs.input", &f_1)?;
    let ret = ListMapTestSumUpArgs {
      input: f_1.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  async fn write_to_out_protocol(&self, o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("sum_up_args");
    o_prot.write_struct_begin(&struct_ident).await?;
    o_prot.write_field_begin(&TFieldIdentifier::new("input", TType::List, 1)).await?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::I32, self.input.len() as i32)).await?;
    for e in &self.input {
      o_prot.write_i32(*e).await?;
      o_prot.write_list_end().await?;
    }
    o_prot.write_field_end().await?;
    o_prot.write_field_stop().await?;
    o_prot.write_struct_end().await
  }
}

//
// ListMapTestSumUpResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ListMapTestSumUpResult {
  result_value: Option<i32>,
}

impl ListMapTestSumUpResult {
  async fn read_from_in_protocol(i_prot: &mut (dyn TAsyncInputProtocol + Send)) -> async_thrift::Result<ListMapTestSumUpResult> {
    i_prot.read_struct_begin().await?;
    let mut f_0: Option<i32> = None;
    loop {
      let field_ident = i_prot.read_field_begin().await?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = i_prot.read_i32().await?;
          f_0 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type).await?;
        },
      };
      i_prot.read_field_end().await?;
    }
    i_prot.read_struct_end().await?;
    let ret = ListMapTestSumUpResult {
      result_value: f_0,
    };
    Ok(ret)
  }
  async fn write_to_out_protocol(&self, o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("ListMapTestSumUpResult");
    o_prot.write_struct_begin(&struct_ident).await?;
    if let Some(fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::I32, 0)).await?;
      o_prot.write_i32(fld_var).await?;
      o_prot.write_field_end().await?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop().await?;
    o_prot.write_struct_end().await
  }
  fn ok_or(self) -> async_thrift::Result<i32> {
    if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        async_thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for ListMapTestSumUp"
          )
        )
      )
    }
  }
}

//
// ListMapTestFindValueArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ListMapTestFindValueArgs {
  input: BTreeMap<i32, i32>,
}

impl ListMapTestFindValueArgs {
  async fn read_from_in_protocol(i_prot: &mut (dyn TAsyncInputProtocol + Send)) -> async_thrift::Result<ListMapTestFindValueArgs> {
    i_prot.read_struct_begin().await?;
    let mut f_1: Option<BTreeMap<i32, i32>> = None;
    loop {
      let field_ident = i_prot.read_field_begin().await?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let map_ident = i_prot.read_map_begin().await?;
          let mut val: BTreeMap<i32, i32> = BTreeMap::new();
          for _ in 0..map_ident.size {
            let map_key_1 = i_prot.read_i32().await?;
            let map_val_2 = i_prot.read_i32().await?;
            val.insert(map_key_1, map_val_2);
          }
          i_prot.read_map_end().await?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type).await?;
        },
      };
      i_prot.read_field_end().await?;
    }
    i_prot.read_struct_end().await?;
    verify_required_field_exists("ListMapTestFindValueArgs.input", &f_1)?;
    let ret = ListMapTestFindValueArgs {
      input: f_1.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  async fn write_to_out_protocol(&self, o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("find_value_args");
    o_prot.write_struct_begin(&struct_ident).await?;
    o_prot.write_field_begin(&TFieldIdentifier::new("input", TType::Map, 1)).await?;
    o_prot.write_map_begin(&TMapIdentifier::new(TType::I32, TType::I32, self.input.len() as i32)).await?;
    for (k, v) in &self.input {
      o_prot.write_i32(*k).await?;
      o_prot.write_i32(*v).await?;
      o_prot.write_map_end().await?;
    }
    o_prot.write_field_end().await?;
    o_prot.write_field_stop().await?;
    o_prot.write_struct_end().await
  }
}

//
// ListMapTestFindValueResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ListMapTestFindValueResult {
  result_value: Option<i32>,
}

impl ListMapTestFindValueResult {
  async fn read_from_in_protocol(i_prot: &mut (dyn TAsyncInputProtocol + Send)) -> async_thrift::Result<ListMapTestFindValueResult> {
    i_prot.read_struct_begin().await?;
    let mut f_0: Option<i32> = None;
    loop {
      let field_ident = i_prot.read_field_begin().await?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = i_prot.read_i32().await?;
          f_0 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type).await?;
        },
      };
      i_prot.read_field_end().await?;
    }
    i_prot.read_struct_end().await?;
    let ret = ListMapTestFindValueResult {
      result_value: f_0,
    };
    Ok(ret)
  }
  async fn write_to_out_protocol(&self, o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("ListMapTestFindValueResult");
    o_prot.write_struct_begin(&struct_ident).await?;
    if let Some(fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::I32, 0)).await?;
      o_prot.write_i32(fld_var).await?;
      o_prot.write_field_end().await?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop().await?;
    o_prot.write_struct_end().await
  }
  fn ok_or(self) -> async_thrift::Result<i32> {
    if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        async_thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for ListMapTestFindValue"
          )
        )
      )
    }
  }
}

