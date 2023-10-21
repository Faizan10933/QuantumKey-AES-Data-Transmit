use chrono;
use config::Config;
use std::collections::HashMap;
use std::env;
use std::io::{ Read, Write };
use std::net::{ Shutdown, TcpListener, TcpStream };
use std::{ thread, time };
use std::process;

use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::str;
type Aes128Cbc = Cbc<Aes128, Pkcs7>;
use std::net::UdpSocket;
use std::net;
use std::fs::OpenOptions;
// use std::io::Write;





#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref KEYLOOKUP: Mutex<HashMap<u32, String>> = Mutex::new(HashMap::new());
}

use aes::cipher::{ generic_array::GenericArray, BlockDecrypt, BlockEncrypt };
use aes::Aes128;
use aes::NewBlockCipher;

static mut KEY_ID: u32 = 0;

use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


fn handle_incoming_key(mut stream: TcpStream) {
    let mut data = [0 as u8; 32];
    let mut data_rcvd = Vec::new();

    while (
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    false
                } else {
                    let d = data.to_vec();
                    data_rcvd.append(&mut d[0..size].to_vec());
                    // println!("=================================]]]]]]]]]]]]]]]]]]]]]]]]]");

                  
                    // println!("{:?}",data_rcvd.to_string());
                    true
                }
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        }
    ) {}
    // println!("{:?}",type_of(data_rcvd));
    // let result_string = String::from_utf8(data_rcvd).expect("Invalid UTF-8");
    let key = std::str::from_utf8(data_rcvd.as_slice()).unwrap();
    // println!("*************************");
    println!("Key Rotation: {:?}",key);


    // let mut map = KEYLOOKUP.lock().unwrap();
    // let key = std::str::from_utf8(data_rcvd.as_slice()).unwrap();

    // unsafe {
    //     KEY_ID += 1;
    //     map.insert(KEY_ID, key.to_string().clone());
    //     println!("Key rotation {:?}", map.get(&KEY_ID).unwrap());
    // }
    // std::mem::drop(map);
}

fn handle_client(mut stream: TcpStream, mut send_stream: TcpStream, write_to_screen: bool) {
    let mut data = [0 as u8; 1200];
    let mut data_rcvd = Vec::new();

    while (
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    false
                } else {

                    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");


                    let key = hex::decode("00000000000000000000000000000000").expect("Decoding failed");
                    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
                    // println!("{:?}",cipher);
                    let mut buf = data;
                    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
                    // println!("!!SDIFHIS");
                    // println!("{:?}",decrypted_ciphertext);
                    // println!("{:?}",decrypted_ciphertext.to_vec());
                    // let my_string = String::from_utf8(decrypted_ciphertext.to_vec()).unwrap();
                    // let my_bytes=my_string.as_bytes();

                    // println!("{}", my_string);


                    // let mut d = data.to_vec();
                    // let chunk_key_id = d[0];
                    // let map = KEYLOOKUP.lock().unwrap();
                    // let chunk_key = map
                    //     .get(&(chunk_key_id as u32))
                    //     .unwrap()
                    //     .clone();

                    // let generic_array_key = GenericArray::clone_from_slice(chunk_key.as_bytes());
                    // let mut generic_array_data = GenericArray::clone_from_slice(&mut d[1..size]);

                    // let cipher = Aes128::new(&generic_array_key);
                    // cipher.decrypt_block(&mut generic_array_data);

                    if write_to_screen == true {
                        data_rcvd.append(&mut decrypted_ciphertext.to_vec());
                    }
                    // println!("Total_data {:?}", std::str::from_utf8(data_rcvd.as_slice()).unwrap());
                    println!(
                        "{:?} {:?} {:?}",
                        chrono::offset::Local::now(),
                        size,
                        " bytes received !"
                    );
                    // std::mem::drop(map);

     //sender/////////////////////////////////////////////

                    // let mut dp = Vec::new();

                    // unsafe {
                    //     let map = KEYLOOKUP.lock().unwrap();

                    //     let chunk_key = map.get(&KEY_ID).unwrap();

                    //     let generic_array_key = GenericArray::clone_from_slice(
                    //         chunk_key.as_bytes()
                    //     );
                    //     let mut generic_array_data = GenericArray::clone_from_slice(
                    //         &mut generic_array_data.to_vec()
                    //     );

                    //     let cipher = Aes128::new(&generic_array_key);
                    //     cipher.encrypt_block(&mut generic_array_data);
                    //     dp.append(&mut vec![KEY_ID as u8]);
                    //     dp.append(&mut generic_array_data.to_vec().clone());
                    //     send_stream.write(&dp.to_vec()).unwrap();

                    //     std::mem::drop(map);
                    // }

                    unsafe{
                    
                        let key = hex::decode("00000000000000000000000000000000").expect("Decoding failed");
                        let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
                        
                        // let pos = my_bytes.len();
                        let mut buffer = [0u8; 1200];
                        buffer[..decrypted_ciphertext.len()].copy_from_slice(decrypted_ciphertext.to_vec().as_slice());
                        let ciphertext = cipher.encrypt(&mut buffer, decrypted_ciphertext.len()).unwrap();

                        println!("sending len: {:?}",ciphertext.len());
                        send_stream.write(&ciphertext.to_vec().as_slice()).unwrap();
                    }

                    true
                }
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        }
    ) {}
    // println!("Total_data {:?}", std::str::from_utf8(data_rcvd.as_slice()).unwrap());
    process::exit(1);
}

fn sender_handle_incoming_key(mut stream: TcpStream) {
    let mut data = [0 as u8; 32];
    let mut data_rcvd = Vec::new();

    while (
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    false
                } else {
                    let d = data.to_vec();
                    data_rcvd.append(&mut d[0..size].to_vec());
                    // println!("size {:?} {:?}", size, &data[0..size]);
                    true
                }
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        }
    ) {}

    // let mut map = KEYLOOKUP.lock().unwrap();
    // let key = std::str::from_utf8(data_rcvd.as_slice()).unwrap();

    // unsafe {
    //     KEY_ID += 1;
    //     map.insert(KEY_ID, key.to_string().clone());
    //     println!("key rotation {:?}", map.get(&KEY_ID).unwrap());
    // }
    let key = std::str::from_utf8(data_rcvd.as_slice()).unwrap();
    // println!("*************************");
    println!("Key Rotation: {:?}",key);
}

fn receiver_handle_incoming_key(mut stream: TcpStream) {
    let mut data = [0 as u8; 32];
    let mut data_rcvd = Vec::new();

    while (
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    false
                } else {
                    let d = data.to_vec();
                    data_rcvd.append(&mut d[0..size].to_vec());
                    true
                }
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        }
    ) {}

    // let mut map = KEYLOOKUP.lock().unwrap();
    // let key = std::str::from_utf8(data_rcvd.as_slice()).unwrap();

    // unsafe {
    //     KEY_ID += 1;
    //     map.insert(KEY_ID, key.to_string().clone());
    //     println!("Key rotation {:?}", map.get(&KEY_ID).unwrap());
    // }
    // std::mem::drop(map);
    let key = std::str::from_utf8(data_rcvd.as_slice()).unwrap();
    // println!("*************************");
    println!("Key Rotation: {:?}",key);
}

fn receiver_handle_client(mut stream: TcpStream, mut socket: &net::UdpSocket) {
    println!("sashdajshd");
    // println("{:?}",stream.to_string());
    let mut data = [0 as u8; 1200];
    // let mut data_rcvd = Vec::new();
    

    while (
        match stream.read(&mut data) {
            Ok(size) => {
                // println!("yoo workings no");
                if size == 0 {
                    // println!("yoo workings nosss");
                    false
                } else {

                    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
                    // println!("yoo working");

                    let key = hex::decode("00000000000000000000000000000000").expect("Decoding failed");
                    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
                    let mut buf = data.to_vec();
                    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
                    // println!("!!SDIFHIS");
                    // println!("{:?}",decrypted_ciphertext.len());
                    // let my_string = String::from_utf8(decrypted_ciphertext.to_vec()).unwrap();
                    // let bytes: &[u8] = decrypted_ciphertext.as_bytes();
                    println!("==========================================================================================================");
                    // let destination = "234.2.2.2:8888";
                    // println!("{}",type_of(&decrypted_ciphertext));
                    // println!("{:?}",decrypted_ciphertext);
                    socket.send_to(decrypted_ciphertext,"234.2.2.2:8888").unwrap();
                    let mut data_file = OpenOptions::new()
                        .append(true)
                        .open("data.ts")
                        .expect("cannot open file");

    // Write to a file
                    data_file
                        .write(decrypted_ciphertext)
                        .expect("write failed");


                    // println!("{}", my_string);
                    // data_rcvd.append(&mut decrypted_ciphertext.to_vec());


                    // let mut d = data.to_vec();
                    // let chunk_key_id = d[0];
                    // let map = KEYLOOKUP.lock().unwrap();
                    // let chunk_key = map.get(&(chunk_key_id as u32)).unwrap();

                    // let generic_array_key = GenericArray::clone_from_slice(chunk_key.as_bytes());
                    // let mut generic_array_data = GenericArray::clone_from_slice(&mut d[1..size]);

                    // let cipher = Aes128::new(&generic_array_key);
                    // cipher.decrypt_block(&mut generic_array_data);

                    // data_rcvd.append(&mut generic_array_data.to_vec());
                    println!(
                        "{:?} {:?} {:?}",
                        chrono::offset::Local::now(),
                        size,
                        " bytes received !"
                    );

                    // std::mem::drop(map);

                    true
                }
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        }
    ) {}
    // println!("Total_data {:?}", std::str::from_utf8(data_rcvd.as_slice()).unwrap());

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    }

    let recv_from = &args[1].parse::<i32>().unwrap();
    let send_to = &args[2].parse::<i32>().unwrap();
    let key_recv_port = &args[3].parse::<i32>().unwrap();
    let b_ip = "127.0.0.1";
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");


    // let socket = UdpSocket::bind("[::]:0");
    // socket.send_to(line.as_bytes(), hostname.to_string() + &":2000")
    //     .expect("Error on send");

    let node = &args[4].parse::<i32>().unwrap(); //

    if *node == 1 {
        let mut map = KEYLOOKUP.lock().unwrap();
        map.insert(0, "0000000000000000".to_string());
        // let ky="0000000000000000";
        std::mem::drop(map);

        println!("Sender");
        let connection_string = format!("{}:{}", b_ip, key_recv_port);
        println!("connecting @ {:#?}", connection_string.clone());

        let thandle_s = thread::spawn(|| {
            let listener = TcpListener::bind(connection_string).unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(move || sender_handle_incoming_key(stream));
                    }
                    Err(_e) => {
                        println!("Failed to connect: {}", _e);
                    }
                }
            }
            drop(listener);
        });

        let connection_string = format!("{}:{}", b_ip, send_to);

        thread::spawn(move || {
            match TcpStream::connect(connection_string.clone()) {
                Ok(mut stream) => {
                    println!("Successfully connected to server in port {:?}", connection_string);
                    let filename: &str = "Untitled.ts";
                    let bytes = std::fs::read(filename).unwrap();

                    for mut bytes in bytes.chunks_exact(1200-1) {
                        let ten_millis = time::Duration::from_millis(1);
                        // let mut message = String::from("Hello wafdafdskfhskdhfkjsdfhkskjfhskdfafdafdjsd sdfsdf");
                        // let mut message = String::from_utf8(bytes.to_vec()).unwrap();

                        // let plaintext=message.as_bytes();
                        thread::sleep(ten_millis);
                        // let mut dp = Vec::new();

                        unsafe {
                            // let map = KEYLOOKUP.lock().unwrap();
                            // let chunk_key = map.get(&KEY_ID).unwrap();
                            // println!("77777777333333333333333333333333333333333333333333");
                            // println!("Key check Rotation: {:?}",key);

                            let key = hex::decode("00000000000000000000000000000000").expect("Decoding failed");
                            let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
                             
                            // let pos = plaintext.len();
                            let mut buffer = [0u8; 1200];
                            buffer[..bytes.len()].copy_from_slice(bytes.to_vec().as_slice());
                            let ciphertext = cipher.encrypt(&mut buffer, bytes.len()).unwrap();

                            println!("sending data len: {:?}",ciphertext.len());
                            stream.write(&ciphertext.to_vec()).unwrap();
                            

                            // let generic_array_key = GenericArray::clone_from_slice(
                            //     chunk_key.as_bytes()
                            // );
                            // let mut generic_array_data = GenericArray::clone_from_slice(&mut bytes);

                            // let cipher = Aes128::new(&generic_array_key);
                            // cipher.encrypt_block(&mut generic_array_data);
                            

                            
                            // println!("{}",type_of(&ciphertext.to_vec()));
                            // println!("{}",type_of(&ciphertext));
                            // println!("{}",type_of(&dp));
                            // println!("{}",type_of(&dp.to_vec()));
                            // dp.append(&mut vec![KEY_ID as u8]);
                            // dp.append(&mut generic_array_data.to_vec().clone());
                            // stream.write(&ciphertext.to_vec()).unwrap();

                            




                        }
                    }
                }
                Err(_e) => {
                    println!("Failed to connect: {}", _e);
                }
            }
        });

        thandle_s.join().unwrap();

        println!("Terminated.");
    } else if *node == 3 {
        println!("end receiver");

        let connection_string = format!("{}:{}", b_ip, recv_from);
        println!("connection string @ {:#?}", connection_string);
      


        // let mut map = KEYLOOKUP.lock().unwrap();
        // map.insert(0, "0000000000000000".to_string());
        
        // std::mem::drop(map);

        
        let thandle = thread::spawn(|| {
            let listener = TcpListener::bind(connection_string).unwrap();
            let socket = UdpSocket::bind("0.0.0.0:9999").unwrap();
            // let socket = UdpSocket::bind("127.0.0.1:8080").unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        // let key = hex::decode("00000000000000000000000000000000").expect("Decoding failed");
                        // let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
                        // let mut buf = stream.to_vec();
                        // let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
                        // println!("{:?}",decrypted_ciphertext);
                        // println!("{:?}",stream);

                        let socket_temp = socket.try_clone().unwrap();
                        println!("Incoming data: {}", stream.peer_addr().unwrap());
                        thread::spawn(move || receiver_handle_client(stream,&socket_temp));
                    }

                    Err(e) => {
                        println!("Error: {}", e);
                        /* connection failed */
                    }
                }
            }
            drop(listener);
        });

        let connection_string = format!("{}:{}", b_ip, key_recv_port);
        println!("{:#?}", connection_string);

        thread::spawn(|| {
            let listener = TcpListener::bind(connection_string).unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("New key received: {}", stream.peer_addr().unwrap());
                        thread::spawn(move || {
                            // connection succeeded

                            receiver_handle_incoming_key(stream)
                        });
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        /* connection failed */
                    }
                }
            }
            drop(listener);
        });

        thandle.join().unwrap();
    } else {
        println!("generic receiver");

        println!("connection string @ {:#?} {:?}", recv_from, send_to);
        let connection_string = format!("{}:{}", b_ip, recv_from);
        let connection_string_send_to = format!("{}:{}", b_ip, send_to);

        let mut map = KEYLOOKUP.lock().unwrap();
        map.insert(0, "0000000000000000".to_string());
        std::mem::drop(map);

        match TcpStream::connect(connection_string_send_to.clone()) {
            Ok(send_stream) => {
                println!(
                    "Successfully connected to server in port {:?}",
                    connection_string_send_to
                );
                thread::spawn(move || {
                    let listener = TcpListener::bind(connection_string).unwrap();
                    for stream in listener.incoming() {
                        let writer = send_stream.try_clone().expect("could not clone ");
                        match stream {
                            Ok(stream) => {
                                println!("Incoming data: {}", stream.peer_addr().unwrap());
                                thread::spawn(move || {
                                    // connection succeeded
                                    handle_client(stream, writer, true)
                                });
                            }

                            Err(e) => {
                                println!("Error: {}", e);
                                /* connection failed */
                            }
                        }
                    }
                    drop(listener);
                });
            }
            Err(_e) => {
                println!("Failed to connect: {}", _e);
            }
        }

        let connection_string = format!("{}:{}", b_ip, key_recv_port);
        println!("{:#?} --------------->", connection_string);

        let thandle_k = thread::spawn(|| {
            let listener = TcpListener::bind(connection_string).unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("New key received: {}", stream.peer_addr().unwrap());
                        thread::spawn(move || {
                            // connection succeeded
                            handle_incoming_key(stream)
                        });
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        /* connection failed */
                    }
                }
            }
            drop(listener);
        });

        thandle_k.join().unwrap();
    }
}