use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap(); // listening at the port

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // we take mut reference because when we read into the buffer some
    // internal state gets modified
    let mut buffer = [0; 1024]; //long enough for our usecase

    stream.read(&mut buffer).unwrap();//populates the buffer with data from the stream
    
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    //Response in the format of: HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
