// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Manager,AppHandle,Emitter};
use std::net::TcpStream;
use std::io::{prelude::*,BufReader}; 
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

#[derive(Default)]
struct AppData{
    username: String,
    tcp_connection:Option<TcpStream>,
    receive_thread:Option<JoinHandle<()>>,
}

fn receive_handler(mut stream:TcpStream,app:AppHandle){
    let mut buf_reader = BufReader::new(&mut stream);
    loop{
        let message: &mut String = &mut Default::default(); 
        let message_error = buf_reader.read_line(message); 
        match message_error {
            Ok(0) => {break;},
            Ok(..) =>{},
            Err(err) => {panic!("Something went wrong while reading the message stream: {:?}",err);}
        };
        println!("Received the following fom the server:{message}"); 
        match app.emit("new_message", message.to_string()){
            Ok(..) => {print!{"Update event emitted for message:{message}"}},
            Err(err) => {print!("Couldn't emit event:{:?}",err);}
        }
    }
    println!("The connection to the server was closed!");
}

#[tauri::command]
fn emit_event(app_handle: tauri::AppHandle){
    println!("emitting!");
    match app_handle.emit("emit",()){
            Ok(..) => {println!{"Update event emitted "}},
            Err(err) => {println!("Couldn't emit event:{:?}",err);}
        }
}

#[tauri::command]
fn connect(address:&str, u_name:&str,state:tauri::State<'_,Mutex<AppData>>,app:AppHandle) -> String {
    println!("Trying to connect to server");
    let stream = TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut ok_stream) => {
                ok_stream.set_nodelay(true).unwrap();
                match state.lock(){
                    Ok(mut state) => {
                        let ok_stream_clone = ok_stream.try_clone().unwrap();
                        state.tcp_connection = Some(ok_stream);
                        state.username = u_name.into();
                        state.receive_thread = Some(thread::spawn(move || {receive_handler(ok_stream_clone,app);}));
                        return "OK".to_string();
                    }
                    Err(error) => {
                        return format!("Couldn't save state:{:?}",error); 
                    }
                }
            }, //mut ok, feels weird
        Err(error) =>{
                println!("An error was thrown when trying to connect{:?}",error);
                return format!("Couldn't connect:{:?}",error); 
            }
    }
}

#[tauri::command]
fn send_message(message:&str,state:tauri::State<'_,Mutex<AppData>>) ->String{
    
    match state.lock(){
        Ok(mut state) => {
            let formatted_message = format!("{0}:{1}\n",state.username,message);
            match state.tcp_connection{
                Some(ref mut stream) => {
                    match stream.write_all(formatted_message.as_bytes()){
                        Ok(..) => {}
                        Err(err) => {
                            println!("[Send_message] {:?}",err);
                            println!("[Send_message] disconnecting from the server");
                            state.tcp_connection = None;
                        }
                    }
                    return "OK".to_string();
                }
                None => {
                    println!("No connection to send anything on yet!");
                    return "err".to_string();
                }
            }
        }
        Err(error)=>{
            println!("An error was thrown while locking state: {:?}",error);
            return "err".to_string();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData::default()));
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![emit_event,connect, send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
