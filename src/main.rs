use std::{
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, TcpListener},
    time::Duration,
};

use ping_rs::send_ping;

fn main() {
    let socket: TcpListener = TcpListener::bind("0.0.0.0:3334").unwrap();
    loop {
        if let Ok((mut conn, _)) = socket.accept() {
            println!("new connection");
            let mut buffer = vec![0_u8; 1 << 9];
            if let Err(_) = conn.read(&mut buffer) {
                println!("error reading from connection");
            }
            if let Ok(content) = String::from_utf8(buffer) {
                println!("{content}");
                let response_lines = content.lines().collect::<Vec<&str>>();
                println!("{}", response_lines[0]);

                let response_content = match response_lines[0].trim_end() {
                    "GET /test HTTP/1.1" => test_server_running(),
                    "GET /test HTTP/1.0" => test_server_running(),

                    "GET /esp_alive HTTP/1.1" => is_esp_up(),
                    "GET /esp_alive HTTP/1.0" => is_esp_up(),
                    "GET /esp_toggle HTTP/1.1" => esp_toggle_power(),
                    "GET /esp_toggle HTTP/1.0" => esp_toggle_power(),
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
        Some(addr) => {
            let request_url = format!("http://{}/", addr);
            match reqwest::blocking::get(request_url) {
                Ok(_) => "HTTP/1.1 200 Ok\r\n\r\nthe esp is alive and present!".to_string(),
                Err(_) => "HTTP/1.1 503 Service Unavailable\r\n\r\nip address present but http server not running???".to_string(),
            }
        }
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
            match reqwest::blocking::get(url) {
                Ok(_) => "HTTP/1.1 200 Ok \r\n\r\n command issued".to_string(),
                Err(_) => "HTTP/1.1 418 I'm a teapot \r\n\r\n command probably issued".to_string(),
            }
        }
    };
}

fn get_esp_address() -> Option<Ipv4Addr> {
    let default_ip = Ipv4Addr::new(192, 168, 0, 14);
    if let Ok(_) = send_ping(
        &IpAddr::V4(default_ip),
        Duration::from_secs(1),
        &[1_u8],
        None,
    ) {
        return Some(default_ip);
    }
    None
}

fn test_server_running() -> String {
    return "HTTP/1.1 200 Ok\r\n\r\nthe server is in deed up".to_string();
}

fn generic() -> String {
    return "HTTP/1.1 400 Bad Request\r\n\r\nstate what you need".to_string();
}
