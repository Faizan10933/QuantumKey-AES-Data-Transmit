let connection_string = format!("{}:{}", b_ip, send_to);

thread::spawn(move || {
    match TcpStream::connect(connection_string.clone()) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port {:?}", connection_string);
            let filename: &str = "test.txt";
            let bytes = std::fs::read(filename).unwrap();

            for mut bytes in bytes.chunks_exact(127) {
                let ten_millis = time::Duration::from_millis(250);
                // let mut message = String::from("Hello wafdafdskfhskdhfkjsdfhkskjfhskdfafdafdjsd sdfsdf");
                let mut message = String::from_utf8(bytes.to_vec()).unwrap();

                let plaintext=message.as_bytes();
                thread::sleep(ten_millis);
                // let mut dp = Vec::new();

                unsafe {
                    // let map = KEYLOOKUP.lock().unwrap();
                    // let chunk_key = map.get(&KEY_ID).unwrap();
                    let key = hex::decode("00000000000000000000000000000000").expect("Decoding failed");
                    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
                     
                    let pos = plaintext.len();
                    let mut buffer = [0u8; 128];
                    buffer[..pos].copy_from_slice(plaintext);
                    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

                    println!("\nCiphertext: {:?}",hex::encode(ciphertext));
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