use std::str::FromStr;
#[derive(Debug)]
pub enum Method
{
    GET,
    POST,
    DELETE,
    PUT,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
    HEAD,
}

impl FromStr for Method 
{
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::DELETE),
            "DELETE" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "PATCH" => Ok(Self::PATCH),
            "TRACE" => Ok(Self::TRACE),
            "CONNECT" => Ok(Self::CONNECT),
            "HEAD" => Ok(Self::HEAD),
            _ => Err(MethodError),
        }

    }
}

pub struct MethodError;