
use std::fs::Metadata;
use std::str::Utf8Error;
use crate::http::{method, request};

use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;
use super::{QueryString};

#[derive(Debug)]
pub struct Request<'buf>
{
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method
}

impl<'buf> Request<'buf>
{
    pub fn path(&self) -> &str { &self.path }

    pub fn method(&self) -> &Method { &self.method }

    pub fn query_string(&self) -> Option<&QueryString>
    {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>
{
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error>
    {

        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1"
        {
            return Err(ParseError::InvalidProtocolError);
        }

        let method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?')
        {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self
            {
                path,
                query_string,
                method
            })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>
{
    for (i, c) in request.chars().enumerate()
    {
        if c == ' ' || c == '\r'
        {
            return Some((&request[..i], &request[(i + 1)..]));
        }
    }
    None
}

pub enum ParseError
{
    InvalidRequest,
    InvalidEncodingError,
    InvalidProtocolError,
    InvalidMethod,
}

impl Display for ParseError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError
{
    fn from(_: Utf8Error) -> Self
    {
        Self::InvalidEncodingError
    }
}

impl From<MethodError> for ParseError
{
    fn from(_: MethodError) -> Self
    {
        Self::InvalidMethod
    }
}



impl ParseError
{
    fn message(&self) -> &str
    {
        match self
        {
            Self::InvalidEncodingError => "Invalid Encoding Error",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocolError => "Invalid Protocol Error",
            Self::InvalidRequest => "Invalid Request",
        }
    }    
}



impl Error for ParseError
{

}