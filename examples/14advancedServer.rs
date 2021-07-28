use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime,UNIX_EPOCH};
use std::str;
use std::{thread, time};
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{Instant, Duration};
use std::io;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const TITLE: &str = "SERVER - AUTHORITATIVE";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;

const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

const PING_TIME: u64 = 5;


fn main() {
    let socket = server_setup(); // make connection w/ socket
    socket.set_read_timeout(None).expect("set_read_timeout call failed");

    let mut client_addresses = HashMap::new(); // store addresses
    let mut player_count: u8 = 1;

    // connecting before game loop!
    'connecting: loop {
        player_count = client_connect(&socket, &mut client_addresses, player_count);
        // increments connection +1
        if player_count == 3 { // if 3, two players are found
            println!("Two players found!");
            break 'connecting;
        }
    }

    for address in client_addresses.keys(){
        socket.send_to(&[0], address).expect("message not sent");
    }

    socket.set_nonblocking(true).unwrap();
    //socket.set_read_timeout(Some(Duration::new(2, 0)));

    run(&socket, &client_addresses);
}

pub fn run(socket: &UdpSocket,
		   client_addresses: &HashMap<SocketAddr, u8>,
		  ) -> Result<(), String>{
   
    let w = 25;
    let x_pos = (CAM_W/2 - w/2) as i32;
    let y_pos = (CAM_H/2 - w/2) as i32;     
    
    let mut p1_box = Rect::new(x_pos, y_pos, w, w);
    let mut p2_box = Rect::new(x_pos, y_pos, w, w);

    let mut p1_x_vel = 0;
    let mut p1_y_vel = 0;
    let mut p2_x_vel = 0;
    let mut p2_y_vel = 0;

    //let received_limit = Duration::from_secs(10);


    'gameloop: loop{
        
        let mut receive_count: u8 = 1;

		let mut input_1 = InputValues{w: false, s: false, a: false, d: false,};
        let mut input_2 = InputValues{w: false, s: false, a: false, d: false,};

		let mut message_1 = false;
        let mut message_2 = false;


        'peeking: loop{
            if ready_to_read(&socket) {break;}
        }

        println!("..\n..\n..\n..\nStarting Receive");

        let receive_time = Instant::now();

        'receiving: loop {
            receive(&socket, &client_addresses, &mut input_1, 
                    &mut input_2, &mut message_1, &mut message_2);
            
            if  //receive_time.elapsed().as_millis() >= Duration::from_millis(PING_TIME).as_millis() || 
                message_1 && message_2 { break; }
        }

        println!("message_1: {}  message_2: {}", message_1, message_2);;

		calc_vel(&input_1, &mut p1_x_vel, &mut p1_y_vel);
		p1_box.set_x(p1_box.x() + p1_x_vel);
		p1_box.set_y(p1_box.y() + p1_y_vel);

		calc_vel(&input_2, &mut p2_x_vel, &mut p2_y_vel);
		p2_box.set_x(p2_box.x() + p2_x_vel);
		p2_box.set_y(p2_box.y() + p2_y_vel);

        let state = GameState::new(p1_box.x(), p1_box.y(), p1_x_vel, p1_y_vel,
        					  	   p2_box.x(), p2_box.y(), p2_x_vel, p2_y_vel);

        send(&socket, &client_addresses, &state);   	
    }

    // Out of game loop, return Ok
    Ok(())
}

fn calc_vel(input: &InputValues, x_vel: &mut i32, y_vel: &mut i32){
    let mut x_deltav = 0;
    let mut y_deltav = 0;
    
    if input.w(){
        y_deltav -= ACCEL_RATE;
    }
    
    if input.a(){
        x_deltav -= ACCEL_RATE;
    }
    
    if input.s() {
        y_deltav += ACCEL_RATE;
    }
    
    if input.d() {
        x_deltav += ACCEL_RATE;
    }

    x_deltav = resist(*x_vel, x_deltav);
    y_deltav = resist(*y_vel, y_deltav);
    
    *x_vel = (*x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
    *y_vel = (*y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
}

fn resist(vel: i32, deltav: i32) -> i32 {
    if deltav == 0 {
        if vel > 0 {
            -1
        }
        else if vel < 0 {
            1
        }
        else {
            deltav
        }
    }
    else {
        deltav
    }
}

fn server_setup() -> UdpSocket{
    // ADDRESSING
    let server_addresses: [SocketAddr; 1] = [
        SocketAddr::from(([127, 0, 0, 1], 1666)),
        // can add backup IPs
    ];

    // BINDING
    let socket = UdpSocket::bind(&server_addresses[..]).expect("couldn't bind to address");
    
    println!("CONNECTED");

    socket
}

fn client_connect(socket: &UdpSocket, 
                  client_addresses: &mut HashMap<SocketAddr,u8>,
                  player_count: u8) -> u8 {
    let mut buffer = [0u8; 100]; // a buffer than accepts 100
    let (number_of_bytes, src_addr) = {
        match socket.recv_from(&mut buffer){
            Ok(t) => t,
            Err(e) => panic!("recv_from function failed: {:?}",e),
        }
    };

    // Client IPs and player #
    if !client_addresses.contains_key(&src_addr) { // for first time
        println!("First time connection to: {:?} > {:?}", src_addr, &buffer[0]); // test to print IP and initial info sent 
        client_addresses.insert(src_addr, player_count); // add to set
        socket.send_to(&[player_count], src_addr); // send player # 
        return player_count + 1; // increment player #
    } 

    return player_count;
}

fn ready_to_read(socket: &UdpSocket) -> bool{
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

fn receive(socket: &UdpSocket,
           client_addresses: &HashMap<SocketAddr, u8>,
           input_1: &mut InputValues,
           input_2: &mut InputValues,
           message_1: &mut bool,
           message_2: &mut bool,
           ){
    let mut buffer = [0u8; 100];

    let (number_of_bytes, src_addr) = {
        match socket.recv_from(&mut buffer){
            Ok(t) => t,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                //println!("Data not ready to be read");
                return;
            }
            Err(e) => panic!("recv_from function failed: {:?}",e),
        }
    };
    
    let mut received_input = deserialize::<InputValues>(&buffer).expect("Couldn't interpret data");
   
    if client_addresses.get(&src_addr).unwrap().eq(&1) && !*message_1{
        input_1.copy(received_input);
        //input_1 = &mut received_input;
        *message_1 = true;
        println!("Received Input from Player 1");
    }else if client_addresses.get(&src_addr).unwrap().eq(&2) && !*message_2{
        input_2.copy(received_input);
        //input_2 = &mut received_input;
        *message_2 = true;
        println!("Received Input from Player 2");
    }
}

fn send(socket: &UdpSocket,
		client_addresses: &HashMap<SocketAddr, u8>,
		state: &GameState){
	let envelope = serialize(state);
	match envelope{
		Ok(encoded_message) =>{ let message = encoded_message.as_slice();
								for address in client_addresses.keys(){
									match socket.send_to(message, address){
                                        Ok(t) => {}//println!("Sent Properly"),
                                        Err(e) => panic!("Couldn't Send: {:?}", e),
                                    }
								}
		},
		Err(e) => panic!("Encoding Failed: {:?}", e),
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState{
    pub p1_x_pos:   i32,
    pub p1_y_pos:   i32,
    pub p1_x_vel:   i32,
    pub p1_y_vel:   i32,    
    pub p2_x_pos:   i32,
    pub p2_y_pos:   i32, 
    pub p2_x_vel:   i32,
    pub p2_y_vel:   i32,
}

impl GameState{
    pub fn new (p1_x_pos: i32, 
                p1_y_pos: i32, 
                p1_x_vel: i32, 
                p1_y_vel: i32, 
                p2_x_pos: i32, 
                p2_y_pos: i32, 
                p2_x_vel: i32, 
                p2_y_vel: i32) -> GameState {
        GameState { p1_x_pos,
                    p1_y_pos,
                    p1_x_vel,
                    p1_y_vel,
                    p2_x_pos,
                    p2_y_pos,
                    p2_x_vel,
                    p2_y_vel,
                }
    }

    pub fn copy(&mut self, other: &GameState){
        self.p1_x_pos   =   other.p1_x_pos();
        self.p1_y_pos   =   other.p1_y_pos();
        self.p1_x_vel   =   other.p1_x_vel();
        self.p1_y_vel   =   other.p1_y_vel();
        self.p2_x_pos   =   other.p2_x_pos();
        self.p2_y_pos   =   other.p2_y_pos();
        self.p2_x_vel   =   other.p2_x_vel();
        self.p2_y_vel   =   other.p2_y_vel();
    }

    pub fn p1_x_pos(&self) -> i32{
        self.p1_x_pos
    }

    pub fn p1_y_pos(&self) -> i32{
        self.p1_y_pos
    }

    pub fn p1_y_vel(&self) -> i32{
        self.p1_y_vel
    }

    pub fn p1_x_vel(&self) -> i32{
        self.p1_x_vel
    }
    pub fn p2_x_pos(&self) -> i32{
        self.p2_x_pos
    }

    pub fn p2_y_pos(&self) -> i32{
        self.p2_y_pos
    }

    pub fn p2_y_vel(&self) -> i32{
        self.p2_y_vel
    }

    pub fn p2_x_vel(&self) -> i32{
        self.p2_x_vel
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputValues{
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,
}

impl InputValues{
    pub fn from_keystate(keystate: &HashSet<Keycode>) -> InputValues {    
        let w = if keystate.contains(&Keycode::W) {
            true
        }else{
            false
        };
        
        let s = if keystate.contains(&Keycode::S) {
            true
        }else{
            false
        };

        let a = if keystate.contains(&Keycode::A) {
            true
        }else{
            false
        };

        let d = if keystate.contains(&Keycode::D) {
            true
        }else{
            false
        };

        InputValues{w,s,a,d,}
    }

    pub fn copy(&mut self, other: InputValues){
        self.w = other.w();
        self.s = other.s();
        self.a = other.a();
        self.d = other.d();
    }

    pub fn w(&self) -> bool{
        self.w
    }

    pub fn s(&self) -> bool{
        self.s
    }

    pub fn a(&self) -> bool{
        self.a
    }

    pub fn d(&self) -> bool{
        self.d
    }
}