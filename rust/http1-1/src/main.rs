use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.3:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conn(stream);
    }
}

pub fn handle_conn(mut stream: TcpStream) {
    let buf = BufReader::new(&mut stream);
    let http_req: Vec<_> = buf
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("{:#?}", http_req);
    if http_req.len() > 0 {
        match http_req[0].as_str() {
            "GET / HTTP/1.1" => {
                let status = "HTTP/1.1 200 OK";
                let content = fs::read_to_string("hello.html").unwrap();
                let content_len = content.len();
                let response =
                    format!("{status}\r\nContent-Length: {content_len}\r\n\r\n{content}");

                stream.write_all(response.as_bytes()).unwrap();
            }
            "GET /bye HTTP/1.1" => {
                let status = "HTTP/1.1 200 OK";
                let content = fs::read_to_string("bye.html").unwrap();
                let content_len = content.len();
                let response =
                    format!("{status}\r\nContent-Length: {content_len}\r\n\r\n{content}");

                stream.write_all(response.as_bytes()).unwrap();
            }

            _ => {
                let status = "HTTP/1.1 404 Not Found";
                let content = fs::read_to_string("nf.html").unwrap();
                let content_len = content.len();
                let response =
                    format!("{status}\r\nContent-Length: {content_len}\r\n\r\n{content}");

                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}
