
use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::convert::TryInto;

pub trait Handler
{
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response
    {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server
{
    addr: String,
}



impl Server
{
    pub fn new(addr: String) -> Self
    {
        Self
        {
            addr: addr
        }
    }

    pub fn run(self, mut handler: impl Handler)
    {
        println!("Listening to {}", self.addr);
    
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop
        {
            let res = listener.accept();
            
            match res
            {
                Ok((mut stream, _)) => 
                {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer)
                    {
                        Ok(_) => 
                        {
                            println!("Received a request: {:?}", String::from_utf8_lossy(&buffer));
                        
                            let response = match Request::try_from(&buffer[..])
                            {
                                Ok(request) => 
                                {
                                    handler.handle_request(&request)
                                },
                                Err(e) =>
                                {
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream)
                            {
                                println!("Failed to send response: {}", e);
                            }

                            let result: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                    
                }

                
                _ => println!("Faile to establish Connection"),
            }
            
        }
    }
}