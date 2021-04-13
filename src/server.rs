pub mod crabserver {

    use std::io::{ErrorKind, Read, Write};
    use std::net::{TcpStream, TcpListener};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn run_server(host: &str) {
        let server = TcpListener::bind(host)
                        .expect("Server failed to bind");

        server.set_nonblocking(true)
        .expect("Failed to initialize server in non-blicking mode");

        let mut clients: Vec<TcpStream> = vec![];
        let (transmitter, reciever) = mpsc::channel::<String>();

        loop {
            if let Ok((mut socket, addr)) = server.accept() {
                println!("{} has connected", addr);

                let transmitter = transmitter.clone();
                clients.push(socket.try_clone().expect("Couldn't clone client"));

                thread::spawn(move ||
                    loop {
                        let mut buff: Vec<u8> = vec![];
                        let timeout: Option<Duration> = Some(Duration::from_millis(50));
                        socket.set_read_timeout(timeout).expect("Failed to establish timeout!");

                        match socket.read(&mut buff) {
                            Ok(size) => {

                                if size == 0 {
                                    //TODO: Figure out how to do below solution without breaking the borrowing rules
                                    // socket.shutdown(Shutdown::Both)
                                    //     .expect("Failed to shutdown socket.");
                                    // let index = clients.iter()
                                    //                 .position(|client| client.peer_addr().unwrap() == addr)
                                    //                 .unwrap();
                                    // clients.remove(index);
                                    // println!("Removing client");
                                    continue;
                                }
                                
                                let msg = buff.into_iter()
                                        .take_while(|&x| x != 0)
                                        .collect::<Vec<_>>();
                                let msg = String::from_utf8(msg)
                                                .expect("Invalid message formatting (UTF-8 expected)");

                                println!("{}: {:?}", addr, msg);
                                transmitter.send(msg).expect("Couldn't send message to the transmitter.");
                            },
                            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                            Err(_) => {
                                println!("An error occured.");
                            }
                        }

                        thread::sleep(Duration::from_millis(100));
                });
            }

            if let Ok(msg) = reciever.try_recv() {
                clients = clients.into_iter().filter_map(|mut client| {
                    let buff = msg.clone().into_bytes();
                    client.write_all(&buff).map(|_| client).ok()
                }).collect::<Vec<_>>();
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}