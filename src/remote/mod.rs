use std::{net::{TcpStream, ToSocketAddrs}, io::{Write, Read}, str::FromStr};

use ascii::AsciiString;
use bytestream::{StreamWriter, ByteOrder, StreamReader};
use prost::Message;

use self::dfproto::EmptyMessage;

#[derive(Debug)]
pub enum RpcCodes{
    RpcReplyResult,
    RpcReplyFail,
    RpcReplyText,
    RpcRequestQuit,
}

impl Into<i16> for RpcCodes{
    fn into(self) -> i16 {
        match self{
            RpcCodes::RpcReplyResult => -1,
            RpcCodes::RpcReplyFail => -2,
            RpcCodes::RpcReplyText => -3,
            RpcCodes::RpcRequestQuit => -4,
        }
    }
}

impl From<i16> for RpcCodes{
    fn from(x: i16) -> Self {
        match x{
            -1 => RpcCodes::RpcReplyResult,
            -2 => RpcCodes::RpcReplyFail,
            -3 => RpcCodes::RpcReplyText,
            -4 => RpcCodes::RpcRequestQuit,
            x => panic!("Invalid Rpc code {}",x)
        }
    }
}

pub mod dfproto{
    tonic::include_proto!("dfproto");
}

#[derive(Debug)]
pub struct RemoteClient{
    connection: TcpStream
}

impl RemoteClient{
    pub fn new<A>(url: A) -> Self
        where A: ToSocketAddrs
    {
        let mut connection = match TcpStream::connect(url){
            Ok(x) => x,
            Err(e) => panic!("failed to connect  to remote :{}",e),
        };

        connection.write(&Self::create_handshake()).unwrap();

        let mut buffer = [0u8;12];

        connection.read_exact(&mut buffer[..]).unwrap();

        if  &buffer[..8] != b"DFHack!\n"{
            panic!("Invalid magic: {}",&std::str::from_utf8(&buffer[..8]).unwrap())
        }

        println!("Successfully connected to df!");

        Self{
            connection
        }
    }

    fn create_handshake() -> [u8;12]{
        let mut out = b"DFHack?\n____".to_owned();
        let x = 1u32.to_le_bytes();
        out[8] = x[0];
        out[9] = x[1];
        out[10] = x[2];
        out[11] = x[3];

        out
    }

    pub fn send_request(&mut self, id: i16, message: impl Message){        
        let data = Self::serialize(id, message);
        self.connection.write_all(&data).unwrap();
    }

    pub fn read_response(&mut self)-> (RpcCodes, Vec<u8>){
        let id: RpcCodes = i16::read_from(&mut self.connection,ByteOrder::LittleEndian).unwrap().into();
        i16::read_from(&mut self.connection, ByteOrder::LittleEndian).unwrap();
        let size = u32::read_from(&mut self.connection, ByteOrder::LittleEndian).unwrap() as usize;

        let mut result_data = vec![0u8;size];
        
        self.connection.read_exact(&mut &mut result_data[..]).unwrap();

        (id,result_data)
    }


    fn send_quit(&mut self){
        self.send_request(RpcCodes::RpcRequestQuit.into(), EmptyMessage{});
    }

    pub fn bind_method(&mut self, name: &str, input: &str, output: &str, plugin: Option<&str>) -> i16{
        let request = dfproto::CoreBindRequest{
            method: AsciiString::from_str(name).unwrap().as_bytes().to_vec(),
            input_msg: AsciiString::from_str(input).unwrap().as_bytes().to_vec(),
            output_msg: AsciiString::from_str(output).unwrap().as_bytes().to_vec(),
            plugin: plugin.map(|x|AsciiString::from_str(x).unwrap().as_bytes().to_vec())
        };

        let buffer = Self::serialize(0, request);
        self.connection.write_all(&buffer).unwrap();
        
        let response = self.read_response();
        match response.0 {
            RpcCodes::RpcReplyResult => {
                
            },
            _ => panic!("failed to bind method {}({}) -> {}",name,input,output)
        }

        let response = dfproto::CoreBindReply::decode(&response.1[..]).unwrap();
        
        response.assigned_id as i16
    }

    fn serialize(id: i16, message: impl Message) -> Vec<u8>{
        
        let mut out = Vec::new();
        id.write_to(&mut out, ByteOrder::LittleEndian).unwrap();
        
        0i16.write_to(&mut out, ByteOrder::LittleEndian).unwrap();

        let data = message.encode_to_vec();
        
        (data.len() as u32).write_to(&mut out, ByteOrder::LittleEndian).unwrap();

        out.write_all(&data).unwrap();

        out
    }

}

impl Drop for RemoteClient {
    fn drop(&mut self) {
        self.send_quit();
        print!("koblet av {:?}",self.connection.peer_addr().unwrap());
    }
}