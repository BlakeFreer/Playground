use askama::Template;
use mario_webserver::mario::*;
use std::str::FromStr;
use std::{
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use strum::IntoEnumIterator;

#[derive(Template)]
#[template(path = "mario.html")]
struct HtmlTemplate<'a> {
    marioname: &'a str,
    transitions: Vec<String>,
    items: Vec<String>,
}

struct Server {
    mario: State,
}

impl Server {
    fn new() -> Server {
        Server {
            mario: State::Alive(Health::Regular),
        }
    }

    fn open(&mut self, ip: &str, port: i16) -> Result<(), io::Error> {
        let address = format!("{ip}:{port}");
        let listener = TcpListener::bind(&address)?;
        println!("Serving on {}.", address);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            if let Err(e) = self.handle_connection(stream) {
                println!("{e}");
            }
        }
        Ok(())
    }

    fn handle_connection(&mut self, mut stream: TcpStream) -> Result<(), String> {
        // Get the http request
        let buf_reader = BufReader::new(&mut stream);
        let request_line = match buf_reader.lines().next() {
            Some(r) => r.expect("Invalid request string"),
            None => return Err(String::from("Request was empty...")),
        };

        // Parse the request to change mario's state
        if let Some(tran) = Self::request_to_transition(request_line) {
            println!("Executing transition {:?}", tran);
            self.mario = transition(&self.mario, tran);
        }

        // Return new HTML with the updated state.
        let name_phrase = match self.mario {
            State::Alive(_) => &format!("It's a me, {}", self.mario),
            State::Dead => "Me's a Dead",
        };

        let html = HtmlTemplate {
            marioname: name_phrase,
            items: Item::iter().map(|x| x.to_string()).collect(),
            transitions: Transition::iter().map(|x| x.to_string()).collect(),
        };
        let html = html.render().expect("Failed to render HTML");
        let length = html.len();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{html}");

        stream.write_all(response.as_bytes()).unwrap();
        Ok(())
    }

    fn request_to_transition(request: String) -> Option<Transition> {
        println!("{request}");
        let mut request = request.split(" ");
        let req_type = request.next().unwrap();
        if req_type != "GET" {
            return None;
        }

        let mut uri = request.next()?.split("/").filter(|x| x != &"");
        let start = uri.next()?;
        if start != "transition" {
            return None;
        }
        match uri.next() {
            Some("item") => Some(Transition::GetItem(
                Item::from_str(uri.next().expect("/item must be followed by an item type"))
                    .unwrap(),
            )),
            Some(t) => Some(Transition::from_str(t).expect("Invalid transition")),
            None => None,
        }
    }
}

fn main() {
    let mut server = Server::new();

    server.open("127.0.0.1", 7878).unwrap();
}
