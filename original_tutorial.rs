// Autogenerated by Thrift Compiler (0.11.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate ordered_float;
extern crate thrift;
extern crate try_from;

use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::From;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use try_from::TryFrom;

use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

//
// Calculator service client
//

pub trait TCalculatorSyncClient {
  /// A method definition looks like C code. It has a return type, arguments,
  /// and optionally a list of exceptions that it may throw. Note that argument
  /// lists and exception lists are specified using the exact same syntax as
  /// field lists in struct or exception definitions.
  fn add(&mut self, num1: i32, num2: i32) -> thrift::Result<i32>;
}

pub trait TCalculatorSyncClientMarker {}

pub struct CalculatorSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  _i_prot: IP,
  _o_prot: OP,
  _sequence_number: i32,
}

impl <IP, OP> CalculatorSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  pub fn new(input_protocol: IP, output_protocol: OP) -> CalculatorSyncClient<IP, OP> {
    CalculatorSyncClient { _i_prot: input_protocol, _o_prot: output_protocol, _sequence_number: 0 }
  }
}

impl <IP, OP> TThriftClient for CalculatorSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  fn i_prot_mut(&mut self) -> &mut TInputProtocol { &mut self._i_prot }
  fn o_prot_mut(&mut self) -> &mut TOutputProtocol { &mut self._o_prot }
  fn sequence_number(&self) -> i32 { self._sequence_number }
  fn increment_sequence_number(&mut self) -> i32 { self._sequence_number += 1; self._sequence_number }
}

impl <IP, OP> TCalculatorSyncClientMarker for CalculatorSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}

impl <C: TThriftClient + TCalculatorSyncClientMarker> TCalculatorSyncClient for C {
  fn add(&mut self, num1: i32, num2: i32) -> thrift::Result<i32> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("add", TMessageType::Call, self.sequence_number());
        let call_args = AddArgs { num1: num1, num2: num2 };
        self.o_prot_mut().write_message_begin(&message_ident)?;
        call_args.write_to_out_protocol(self.o_prot_mut())?;
        self.o_prot_mut().write_message_end()?;
        self.o_prot_mut().flush()
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin()?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("add", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
        self.i_prot_mut().read_message_end()?;
        return Err(thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = AddResult::read_from_in_protocol(self.i_prot_mut())?;
      self.i_prot_mut().read_message_end()?;
      result.ok_or()
    }
  }
}

//
// Calculator service processor
//

pub trait CalculatorSyncHandler {
  /// A method definition looks like C code. It has a return type, arguments,
  /// and optionally a list of exceptions that it may throw. Note that argument
  /// lists and exception lists are specified using the exact same syntax as
  /// field lists in struct or exception definitions.
  fn handle_add(&self, num1: i32, num2: i32) -> thrift::Result<i32>;
}

pub struct CalculatorSyncProcessor<H: CalculatorSyncHandler> {
  handler: H,
}

impl <H: CalculatorSyncHandler> CalculatorSyncProcessor<H> {
  pub fn new(handler: H) -> CalculatorSyncProcessor<H> {
    CalculatorSyncProcessor {
      handler: handler,
    }
  }
  fn process_add(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    TCalculatorProcessFunctions::process_add(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
}

pub struct TCalculatorProcessFunctions;

impl TCalculatorProcessFunctions {
  pub fn process_add<H: CalculatorSyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let args = AddArgs::read_from_in_protocol(i_prot)?;
    match handler.handle_add(args.num1, args.num2) {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("add", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident)?;
        let ret = AddResult { result_value: Some(handler_return) };
        ret.write_to_out_protocol(o_prot)?;
        o_prot.write_message_end()?;
        o_prot.flush()
      },
      Err(e) => {
        match e {
          thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("add", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("add", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
        }
      },
    }
  }
}

impl <H: CalculatorSyncHandler> TProcessor for CalculatorSyncProcessor<H> {
  fn process(&self, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let message_ident = i_prot.read_message_begin()?;
    let res = match &*message_ident.name {
      "add" => {
        self.process_add(message_ident.sequence_number, i_prot, o_prot)
      },
      method => {
        Err(
          thrift::Error::Application(
            ApplicationError::new(
              ApplicationErrorKind::UnknownMethod,
              format!("unknown method {}", method)
            )
          )
        )
      },
    };
    thrift::server::handle_process_result(&message_ident, res, o_prot)
  }
}

//
// AddArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct AddArgs {
  num1: i32,
  num2: i32,
}

impl AddArgs {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<AddArgs> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i32> = None;
    let mut f_2: Option<i32> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i32()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_i32()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("AddArgs.num1", &f_1)?;
    verify_required_field_exists("AddArgs.num2", &f_2)?;
    let ret = AddArgs {
      num1: f_1.expect("auto-generated code should have checked for presence of required fields"),
      num2: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("add_args");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("num1", TType::I32, 1))?;
    o_prot.write_i32(self.num1)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("num2", TType::I32, 2))?;
    o_prot.write_i32(self.num2)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// AddResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct AddResult {
  result_value: Option<i32>,
}

impl AddResult {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<AddResult> {
    i_prot.read_struct_begin()?;
    let mut f_0: Option<i32> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = i_prot.read_i32()?;
          f_0 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = AddResult {
      result_value: f_0,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("AddResult");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::I32, 0))?;
      o_prot.write_i32(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
  fn ok_or(self) -> thrift::Result<i32> {
    if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for Add"
          )
        )
      )
    }
  }
}

