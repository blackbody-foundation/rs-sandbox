use std::{error, io, net, str};

use console;
use crossbeam;

fn main() -> Result<(), Box<dyn error::Error>> {
    let client_addr = "127.0.0.1:5451";
    let server_addr = "127.0.0.1:8888";

    let term = console::Term::stdout();
    term.write_line("Please 'Enter' to send a message.")?;

    crossbeam::thread::scope(|s| {
        // server role
        s.spawn(|_| -> io::Result<()> {
            loop {
                let udp_socket = net::UdpSocket::bind(server_addr)?;

                let mut buf = [0; 100];
                let (amount, _) = udp_socket.recv_from(&mut buf)?;
                let s = str::from_utf8(&buf[..amount]).unwrap();
                term.write_line(&format!("Received!: {s}\r"))?;
            }
        });

        // client role
        s.spawn(|_| -> io::Result<()> {
            loop {
                let a = term.read_key()?;
                if a == console::Key::Enter {
                    let message = "Test Message";

                    term.write_line(&format!("Send! {message}"))?;

                    let udp_socket = net::UdpSocket::bind(client_addr)?;

                    udp_socket.send_to(message.as_bytes(), server_addr)?;
                }
            }
        });
    })
    .unwrap();

    Ok(())
}
