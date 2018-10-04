use config::Config;
use futures::Future;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownFrameType(String),
    InvalidFieldFormat(String),
    UnexpectedResponse,
    OutOfOrderRequest,
}

#[derive(Debug, PartialEq)]
pub enum RequestError {
    UnknownRequestType(String),
    UnknownMandatoryHeaders(Vec<String>),
    MalformedHeader(String),
    MalformedField(String),
}

impl RequestError {
    pub fn status(&self) -> Status {
        match *self {
            RequestError::UnknownRequestType(_) => Status::SE(2),
            RequestError::UnknownMandatoryHeaders(_) => Status::SE(1),
            RequestError::MalformedHeader(_) => Status::SE(0),
            RequestError::MalformedField(_) => Status::SE(0),
        }
    }
}

pub trait FrameHandler<Frame, Req, Res>
where
    Self: Sized,
{
    fn new(config: Config<Req, Res>) -> (Self, Arc<Mutex<ResponseFrameSource<Frame>>>);
    fn handle(&mut self, frame: Frame) -> Result<Option<Frame>, Error>;
}

#[derive(Debug)]
pub enum BodyError {
    Missing,
    Invalid,
}

pub trait IntoFrame<F> {
    fn into_frame(self, id: u32) -> F;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Status {
    OK(u8),
    SE(u8),
    RE(u8),
}

pub trait ResponseFrameSource<F> {
    fn on_response_frame(&mut self, frame_id: u32) -> Box<Future<Item = F, Error = ()>>;
}