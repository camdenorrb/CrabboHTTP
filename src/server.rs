
use async_std::net::{TcpListener, TcpStream};

use async_std::task;
use http_types::{Response, StatusCode, Version};
use async_std::{io::{prelude::*, BufReader}, prelude::*};
use itertools::Itertools;


pub async fn main() -> http_types::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut incoming = listener.incoming();

    while let Some(stream_result) = incoming.next().await {

        let mut stream = stream_result?;

        task::spawn(async move {
            handle_connection(&mut stream).await.expect("Error: Handling connection gone wrong.");
        });
    }

    Ok(())
}

async fn handle_connection(stream: &mut TcpStream) -> http_types::Result<()> {

    //let (reader, writer) = &mut (&stream, &stream);

    let mut res = Response::new(StatusCode::Ok);

    res.set_version(Some(Version::Http1_1));
    res.insert_header("Content-Type", "text/html")?;

    let request = read_request(&stream).await?;

    res.set_body(format!(
        "<html>\n\
            \t<pre>{request}</pre>\n\
         </html>
        ", request = request)
    );

    encode_response(&mut res).await?;

    //println!("{}", string);

    //println!("{}", res);

    //stream.write_all(("HTTP/1.1 200 OK\r\n\r\n".to_string() + "Meow").as_bytes()).await?;
    //println!("{}", encode_response(&mut res).await?);
    stream.write_all(encode_response(&mut res).await?.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

async fn encode_response(response: &mut Response) -> http_types::Result<String> {

    let version = response.version().expect("No version in response?!");

    let status  = response.status();

    // Parses stuff like HTTP1_0 to 1.0
    let version_number = &format!("{:?}", version)["HTTP".len()..].replace("_", ".");

    let mut body_text = String::new();

    response.read_to_string(&mut body_text).await?;

    let content_headers = response.iter().map(|it| format!("{}: {}", it.0, it.1.iter().join(", "))).join("\n");

    let response_text = format!("HTTP/{} {} {} \n{}\r\n\r\n{}", version_number, status, status.canonical_reason(), content_headers, body_text);

    // Rewrite body to original
    response.set_body(body_text);

    Ok(response_text)
}


async fn read_request(stream: &TcpStream) -> async_std::io::Result<String> {

    let reader = BufReader::new(stream);

    let mut output = String::new();
    let mut lines = reader.lines();

    while let Some(line) = lines.next().await {

        let unwrapped = line?;

        if unwrapped == "" {
            break
        }

        output += &(unwrapped + "\n");
    }

    Ok(output)
}