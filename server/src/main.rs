//The server will consis of 1 big text file displayed on the UI.
//The client will only send a username and a message, these will be formatted in some way and sent
////to the other clients. 
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex},
    thread,
};

fn sender(rx:Receiver<String>, streams_mux:Arc<Mutex<Vec<TcpStream>>>){
    loop{
        let message = match rx.recv(){
            Ok(message) => message,
            Err(error) => {
                println!("Error while reading channel, closing thread:{:?}",error);
                return;
            }
        };  
        let mut streams = streams_mux.lock().unwrap();
        for i in 0..streams.len(){
            match streams.get(i){
                Some(mut stream) => {
                    match stream.write_all(message.as_bytes()){
                        Ok(..)=>{}
                        Err(..)=>{}
                    };
                                    },
                None => {
                    println!("Failed to write to client:Stream was None"); 
                    println!("Failed to write to client:couldn't get stream. Removing entry");
                    streams.remove(i);

                }
            };
        }
    }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //multi-producer, single consumer. Its all built in here :o
    let (sender_tx,sender_rx) :(Sender<String>,Receiver<String>) = mpsc::channel();
    let mut network_streams: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));
    let mut network_streams_clone = network_streams.clone();
    let sender_handler: thread::JoinHandle<()> = thread::spawn(move || {sender(sender_rx,network_streams_clone)});
    
    let mut join_handlers: Vec<thread::JoinHandle<()>> = vec![];

    for stream in listener.incoming() {
        let thread_tx = sender_tx.clone();
        let stream = stream.unwrap();
        
        network_streams.lock().unwrap().push(stream.try_clone().unwrap());
        
        println!("New connection incomming!");
        //One thread per connection lets go!!
        join_handlers.push(thread::spawn(move || {
            connection_listener(stream,thread_tx);
        }));
    }

    for handler in join_handlers.into_iter(){
        handler.join().expect("Handler thread panicked");
    }
}

fn connection_listener(mut stream: TcpStream,tx:Sender<String>) {
    let mut buf_reader = BufReader::new(&mut stream);
    loop{
        let message: &mut String = &mut Default::default(); 
        println!("waiting for a message");
        let message_error = buf_reader.read_line(message); 
        match message_error {
            Ok(0) => {break;},
            Ok(..) =>{},
            Err(err) => {
                println!("Something went wrong while reading the message stream: {:?}",err);
                break;
            }
        };
        println!("Received the following fom the client:{message}"); 
        tx.send(message.to_string()).unwrap();
        //Add message to the message messageQueue
    }
    println!("The connection to the client was closed!");
}
