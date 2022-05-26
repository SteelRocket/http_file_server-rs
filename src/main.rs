extern crate tiny_http;

use std::path::Path;
use ascii::AsciiString;

const DIR_HTML: &str = std::include_str!("./dir.min.html");

fn format_dir_html(url: String) -> String {

    let html = DIR_HTML.replace("R{dir}", url.as_str());
    
    let paths = std::fs::read_dir(url.clone()).unwrap();
    let mut html_lists = "".to_string();

    for path in paths {
        let path_buf = path.unwrap().path();
        let file = path_buf.to_str().unwrap();

        html_lists.push_str(
            format!(
                "<li><a href=\"{}\">{}</li>",
                file.replacen(".", "", 1), //? Required as "./" would be relative path so we remove the '.'
                file.replace(&url, "") // Gets the file name
            ).as_str()
        );
    }
    html.replace("R{files}", html_lists.as_str())
}

fn send_respond_file(request: tiny_http::Request, url: String){
    let path = Path::new(&url);

    if !path.exists() { // Return 404 error
        request.respond(
        tiny_http::Response::new_empty(tiny_http::StatusCode(404))
        ).unwrap();
        return;
    }

    if path.is_dir(){ // Return dir.html

        let response = tiny_http::Response::from_string(format_dir_html(url));
        let response = response.with_header(tiny_http::Header {
            field: "Content-Type".parse().unwrap(),
            value: AsciiString::from_ascii("text/html; charset=utf8").unwrap(),
        });

        request.respond(
            response
        ).unwrap();

        return;
    }

    let response = tiny_http::Response::from_file( // Return the file requested
        std::fs::File::open(path).unwrap()
    );
    request.respond(response.with_header(
        tiny_http::Header {
            field: "Content-Type".parse().unwrap(),
            value: AsciiString::from_ascii("text/plain; charset=utf8").unwrap(),
        }
    )).unwrap();
}

fn main() {
    let server = tiny_http::Server::http("0.0.0.0:9842").unwrap();
    println!("Now listening on http://127.0.0.1:9842"); // 0.0.0.0 is viewable using 127.0.0.1

    for return_req in server.incoming_requests() {
        let mut url = ".".to_string() + return_req.url();

        match url.chars().position(|c| c == '?') { // Remove query parameters from url
            Some(i) => {
                url = url[0..i].to_string();
            },
            _ => {

            },
        };
        url = url.replace("%20", " ");

        send_respond_file(return_req, url);
    }
}