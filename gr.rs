fn handle_client(mut stream: TcpStream, mut send_stream: TcpStream, write_to_screen: bool) {
    let mut data = [0 as u8; 128];
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
                    let my_string = String::from_utf8(decrypted_ciphertext.to_vec()).unwrap();
                    let my_bytes=my_string.as_bytes();

                    println!("{}", my_string);


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
                        
                        let pos = my_bytes.len();
                        let mut buffer = [0u8; 128];
                        buffer[..pos].copy_from_slice(my_bytes);
                        let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

                        println!("\nCiphertext: {:?}",hex::encode(ciphertext));
                        stream.write(&ciphertext.to_vec()).unwrap();
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
    println!("Total_data {:?}", std::str::from_utf8(data_rcvd.as_slice()).unwrap());
    process::exit(1);
}