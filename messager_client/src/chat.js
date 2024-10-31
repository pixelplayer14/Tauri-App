const {invoke} = window.__TAURI__.core;
const {listen} = window.__TAURI__.event;


async function sendMessage(message){
    await invoke("send_message", { message: message});
}

window.addEventListener("DOMContentLoaded", () => {
    let messageInput = document.querySelector("#message-input");
    let messageBox = document.querySelector("#msg-box");
    document.querySelector("#message-form").addEventListener("submit", (e) => {
	e.preventDefault();
	console.log(`trying to send message:${messageInput.value}`);
	sendMessage(messageInput.value); 
    });
    
    let new_message_handler = listen('new_message', (e) => {
	let message = e.payload
	let messageTag = document.createElement("p");
	let text = document.createTextNode(message);
	messageBox.appendChild(messageTag)
	messageTag.appendChild(text);
    });
});
