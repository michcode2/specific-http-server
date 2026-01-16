use std::{
    io::{Read, Write},
    net::TcpListener,
    time::Duration,
};

use ping_rs::{PingOptions, send_ping};

fn main() {
    let socket: TcpListener = TcpListener::bind("0.0.0.0:3334").unwrap();
    loop {
        if let Ok((mut conn, _)) = socket.accept() {
            println!("new connection");
            let mut buffer = vec![0_u8; 1 << 9];
            if let Err(_) = conn.read(&mut buffer) {
                println!("error reading from connection");
            }
            println!("{:?}", buffer);
            if let Ok(content) = String::from_utf8(buffer) {
                println!("{content}");
                let response_lines = content.lines().collect::<Vec<&str>>();
                println!("{}", response_lines[0]);

                let response_content = match response_lines[0] {
                    "GET /test HTTP/1.1" => test_server_running(),
                    "GET /esp_alive HTTP/1.1" => is_esp_up(),
                    "GET /esp_toggle HTTP/1.1" => esp_toggle_power(),
                    _ => generic(),
                };
                if let Err(_) = conn.write(response_content.as_bytes()) {
                    println!("error writing to connection");
                }
                continue;
            }
            if let Err(_) = conn.write("HTTP/1.1 400 OK\r\n\r\nidk what happened xD\r\n".as_bytes())
            {
                println!("error writing to connection");
            }
        }
    }
}

fn is_esp_up() -> String {
    return match get_esp_address() {
        Some(_) => "HTTP/1.1 200 Ok\r\n\r\nthe esp is alive and present!".to_string(),
        None => {
            "HTTP/1.1 503 Service Unavailable\r\n\r\ncould not find the esp at known addresses :("
                .to_string()
        }
    };
}

fn esp_toggle_power() -> String {
    return match get_esp_address() {
        None => {
            "HTTP/1.1 503 Service Unavailable\r\n\r\ncould not find the esp at known addresses :("
                .to_string()
        }
        Some(address) => {
            let url = format!("http://{}/balls", address);
            println!("{url}");
            reqwest::blocking::get(url).unwrap();
            "HTTP/1.1 200 Ok \r\n\r\n command issued".to_string()
        }
    };
}

fn get_esp_address() -> Option<String> {
    if let Ok(_) = send_ping(
        &"192.168.0.14".parse().unwrap(),
        Duration::from_secs(1),
        &[1_u8],
        None,
    ) {
        return Some("192.168.0.14".to_string());
    }

    if let Ok(_) = send_ping(
        &"pcturnon.local".parse().unwrap(),
        Duration::from_secs(1),
        &[1_u8],
        None,
    ) {
        return Some("pcturnon.local".to_string());
    }
    None
}

fn test_server_running() -> String {
    return "HTTP/1.1 200 Ok\r\n\r\nthe server is in deed up".to_string();
}

fn generic() -> String {
    return "HTTP/1.1 400 Bad Request\r\n\r\nstate what you need".to_string();
}
