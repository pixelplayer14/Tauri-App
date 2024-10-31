const { invoke } = window.__TAURI__.core;
const {listen} = window.__TAURI__.event;

let greetInputEl;
let greetMsgEl;

let messageInputEl;
let messageFeedbackEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

let new_message_handler = listen('emit', () => {
    console.log('received');
});


async function connect(name){
    return await invoke("connect",{address:"127.0.0.1:8080",uName:name});
}


window.addEventListener("DOMContentLoaded", () => {
    let loginInputEL= document.querySelector("#login-input");
    let loginFeedbackEL = document.querySelector("#login-feedback");
    let eventButton = document.querySelector("#event");
    console.log(loginInputEL,loginFeedbackEL);
    document.querySelector("#login-form").addEventListener("submit", (e) => {
	e.preventDefault();
    	connect(loginInputEL.value).then((value)=>{
	    loginFeedbackEL.innerText=value
	    if(value == "OK"){
		window.location.href="chat.html";
	    }
	    console.log("Login attempt result:",value);
	});
     });
     eventButton.addEventListener("click", (e)=>{ invoke("emit_event",{}); });
});
