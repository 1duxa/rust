use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
};

#[allow(dead_code)]
struct Responses {
    ok_resp: &'static str,
    bad_resp: &'static str,
}

impl Responses {
    pub fn new() -> Self {
        Self {
            ok_resp: "HTTP/1.1 200 OK\r\n",
            bad_resp: "HTTP/1.1 404 Not Found\r\n",
        }
    }

    pub fn write_not_found_err(&self, stream: &mut TcpStream) {
        let mut file = File::open("./error_html/nf.html").unwrap();
        let mut response_body = String::new();
        let _ = file.read_to_string(&mut response_body);
        let response = format!(
            "{}Content-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
            self.bad_resp,
            response_body.len(),
            response_body
        );
        let _ = stream.write_all(response.as_bytes());
    }

    pub fn write_err(&self, stream: &mut TcpStream, error: &str) {
        let response = format!("{}{}", self.bad_resp, error);
        let _ = stream.write_all(response.as_bytes());
    }
}

type FnForPath = Box<dyn FnMut(&mut TcpStream)>;
#[derive(Default)]
pub struct PathListener {
    paths: HashMap<String, FnForPath>,
}

impl PathListener {
    pub fn add_route<F>(&mut self, path: &str, handler: F)
    where
        F: FnMut(&mut TcpStream) + 'static,
    {
        self.paths.insert(path.to_string(), Box::new(handler));
    }

    pub fn handle_request(&mut self, path: &str, stream: &mut TcpStream) -> bool {
        if let Some(handler) = self.paths.get_mut(path) {
            handler(stream);
            true
        } else {
            false
        }
    }
}

enum HttpTypes {
    HttpOne,
    HttpTwo,
}

enum Headers {
    ContentLength(usize),
    ContentType(String),
}

struct ResponseBuilder(HttpTypes);

impl ResponseBuilder {
    pub fn new(http_type: HttpTypes) -> Self {
        Self(http_type)
    }

    pub fn build(&self, status: &str, headers: Vec<Headers>, body: &str) -> String {
        let mut response = match self.0 {
            HttpTypes::HttpOne => format!("HTTP/1.1 {}\r\n", status),
            HttpTypes::HttpTwo => format!("HTTP/2 {}\r\n", status),
        };

        for header in headers {
            match header {
                Headers::ContentLength(len) => {
                    response.push_str(&format!("Content-Length: {}\r\n", len))
                }
                Headers::ContentType(ctype) => {
                    response.push_str(&format!("Content-Type: {}\r\n", ctype))
                }
            }
        }
        response.push_str(format!("\r\n{}", body).as_str());
        response
    }
}

pub struct App {
    listener: TcpListener,
    pub path_listener: PathListener,
}

impl App {
    pub fn new(addr: &str) -> Self {
        Self {
            listener: TcpListener::bind(addr).unwrap(),
            path_listener: PathListener::default(),
        }
    }

    pub fn serve<F>(&mut self, mut logic: F)
    where
        F: FnMut(Vec<String>, &mut TcpStream) + 'static,
    {
        let resps = Responses::new();
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut ok_stream) => {
                    let mut reader = BufReader::new(&mut ok_stream);
                    let mut request = Vec::new();

                    loop {
                        let mut line = String::new();
                        match reader.read_line(&mut line) {
                            Ok(0) => {
                                eprintln!("ERS: 0b");
                                resps.write_err(&mut ok_stream, "Empty request");
                                break;
                            }
                            Ok(_) => {
                                if line == "\r\n" {
                                    break;
                                }
                                request.push(line.replace("\r\n", ""));
                            }
                            Err(e) => {
                                eprintln!("ERS: {}", e);
                                resps.write_err(&mut ok_stream, &e.to_string());
                                break;
                            }
                        }
                    }

                    if !request.is_empty() {
                        println!("--> \n {:#?}", request);

                        if let Some(first_line) = request.first() {
                            let parts: Vec<&str> = first_line.split(' ').collect();
                            if parts.len() > 1 {
                                let path = parts[1];
                                if !self.path_listener.handle_request(path, &mut ok_stream) {
                                    logic(request, &mut ok_stream);
                                }
                            } else {
                                resps.write_err(&mut ok_stream, "400 Bad Request");
                            }
                        }
                    } else {
                        resps.write_err(&mut ok_stream, "Request is empty");
                    }
                }
                Err(e) => {
                    println!("Error Matching Stream: {}", e);
                    break;
                }
            }
        }
    }

    pub fn add_route<F>(&mut self, path: &str, handler: F)
    where
        F: FnMut(&mut TcpStream) + 'static,
    {
        self.path_listener.add_route(path, handler);
    }
}

#[allow(unused)]
fn main() {
    let mut app = App::new("127.0.0.1:4221");
    app.add_route("/hello", |stream| {
        let body = "Hello, world!";
        let response = ResponseBuilder::new(HttpTypes::HttpOne).build(
            "200 OK",
            vec![
                Headers::ContentLength(body.len()),
                Headers::ContentType("text/plain".to_string()),
            ],
            body,
        );
        let _ = stream.write_all(response.as_bytes());
    });
    app.add_route("/gg", |stream| {
        let body = "gg";
        let response = ResponseBuilder::new(HttpTypes::HttpOne).build(
            "200 OK",
            vec![
                Headers::ContentLength(body.len()),
                Headers::ContentType("text/plain".to_string()),
            ],
            body,
        );
        let _ = stream.write_all(response.as_bytes());
    });
    app.serve(Box::new(
        |_request: Vec<String>, _stream: &mut TcpStream| {},
    ));
}
