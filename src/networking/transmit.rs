use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::collections::HashMap;
use std::collections::HashSet;
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize};

pub fn receive_input(socket: &UdpSocket,
				  client_addresses: &HashMap<SocketAddr, u8>,
				  input_1: &mut HashSet<u8>,
				  input_2: &mut HashSet<u8>,
				  message_1: &mut bool,
				  message_2: &mut bool,
				  ){
	let mut buffer = [0u8; 100];

	let (number_of_bytes, src_addr) = {
		match socket.recv_from(&mut buffer){
			Ok(t) => t,
			Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
				return;
			}
			Err(e) => panic!("recv_from function failed: {:?}", e),
		}
	};

    let received_input = deserialize::<HashSet<u8>>(&buffer).expect("Couldn't interpret data");
   
    if client_addresses.get(&src_addr).unwrap().eq(&1) && !*message_1{
        print!("received: ");
    	for keys in received_input.iter(){
    		print!(" {:?},", *keys);
    		input_1.insert(*keys);
    	}        
    	println!("");
        *message_1 = true;
        println!("Received Input from Player 1");
    }else if client_addresses.get(&src_addr).unwrap().eq(&2) && !*message_2{
		for keys in received_input.iter(){
			input_2.insert(*keys);
		}        
        *message_2 = true;
        println!("Received Input from Player 2");
    }
}

pub fn send_input(socket: &UdpSocket, inputs: &HashSet<u8>,){
	let envelope = serialize(inputs);
    match envelope{
        Ok(encoded_message) =>{ let message = encoded_message.as_slice();
                                socket.send(message);},
        Err(e) => panic!("Send Failed: {:?}", e),
    }
}

pub fn send_game_state(){
}

pub fn receive_game_state(){
}

pub fn ready_to_read(socket: &UdpSocket) -> bool{
    let mut buffer = [0u8; 100];
    match socket.peek(&mut buffer){
        Ok(t) => t,
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            //println!("not ready to peak");
            return false;
        }
        Err(e) =>{ 
            panic!("peek function failed: {:?}", e); 
            return false;
        }
    };
    return true
}