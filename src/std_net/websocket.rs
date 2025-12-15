use crate::{prelude::*, std_err::WrappedError, std_json};
use mluau::prelude::*;

use url::Url;

use std::net::TcpStream;
use tungstenite::{Message, stream::MaybeTlsStream};

struct WebsocketMessage {
    inner: Message,
}
impl WebsocketMessage {
    fn from(message: Message) -> Self {
        Self {
            inner: message
        }
    }
    fn get_userdata(self, luau: &Lua) -> LuaValueResult {
        ok_userdata(self, luau)
    }
}
impl LuaUserData for WebsocketMessage {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "WebsocketMessage");
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("is_binary", |_luau, it, _: LuaValue| {
            if let Message::Binary(_) = it.inner {
                Ok(true)
            } else {
                Ok(false)
            }
        });
        methods.add_method("is_utf8", |_luau, it, _: LuaValue| {
            if let Message::Text(_) = it.inner {
                Ok(true)
            } else {
                Ok(false)
            }
        });
        methods.add_method("type", |luau, it, _: LuaValue| {
            let t = match it.inner {
                Message::Binary(_) => "Binary",
                Message::Text(_) => "Text",
                Message::Close(_) => "Close",
                Message::Ping(_) => "Ping",
                Message::Pong(_) => "Pong",
                Message::Frame(_) => "Frame",
            };
            t.into_lua(luau)
        });
        methods.add_method_mut("as_bytes", |luau, it, _: LuaValue| {
            let bytes = match &it.inner {
                Message::Binary(bytes) => bytes.to_vec(),
                Message::Text(s) => s.as_str().as_bytes().to_owned(),
                Message::Close(b) => {
                    if let Some(frame) = b {
                        let info = format!("[CLOSE] CODE {} REASON {}", frame.code, frame.reason);
                        info.as_bytes().to_vec()
                    } else {
                        b"".to_vec()
                    }
                },
                Message::Ping(bytes) | Message::Pong(bytes) => bytes.to_vec(),
                Message::Frame(frame) => {
                    frame.payload().to_vec()
                }
            };

            ok_buffy(bytes, luau)
        });
        methods.add_method_mut("as_string", |luau, it, _: LuaValue| {
            let bytes = match &it.inner {
                Message::Binary(bytes) => bytes.to_vec(),
                Message::Text(s) => s.as_str().as_bytes().to_owned(),
                Message::Close(b) => {
                    if let Some(frame) = b {
                        let info = format!("[CLOSE] CODE {} REASON {}", frame.code, frame.reason);
                        info.as_bytes().to_vec()
                    } else {
                        b"".to_vec()
                    }
                },
                Message::Ping(bytes) | Message::Pong(bytes) => bytes.to_vec(),
                Message::Frame(frame) => {
                    frame.payload().to_vec()
                }
            };

            ok_string(bytes, luau)
        });
        methods.add_method_mut("try_json", |luau, it, _: LuaValue| {
            let function_name = "WebSocketMessage:expect_json()";
            let text = match &it.inner {
                Message::Text(text) => text.to_string(),
                other => {
                    let error_message = format!("{}: expected Message::Text, got: {:?}", function_name, other);
                    return WrappedError::with_traceback(error_message, luau)?.get_userdata(luau);
                }
            };

            let decoded = match crate::std_json::json_decode(luau, text) {
                Ok(decoded) => decoded,
                Err(err) => {
                    let error_message = format!("{}: decoding error: {}", function_name, err);
                    return WrappedError::with_traceback(error_message, luau)?.get_userdata(luau)
                }
            };

            Ok(decoded)
        });
        methods.add_method_mut("expect_json", |luau, it, _: LuaValue| {
            let function_name = "WebSocketMessage:expect_json()";
            let text = match &it.inner {
                Message::Text(text) => text.to_string(),
                other => {
                    return wrap_err!("{}: message is not Message::Text (got: {:?})", function_name, other);
                }
            };

            let decoded = match crate::std_json::json_decode(luau, text) {
                Ok(decoded) => decoded,
                Err(err) => {
                    return wrap_err!("{}: decode error: {}", function_name, err);
                }
            };

            Ok(decoded)
        });
    }
}

type TungsteniteWebSocket = tungstenite::WebSocket<MaybeTlsStream<TcpStream>>;

struct WebsocketWrapper {
    inner: TungsteniteWebSocket
}
impl WebsocketWrapper {
    fn new(inner: TungsteniteWebSocket) -> Self {
        Self {
            inner
        }
    }
    fn read(&mut self) -> LuaResult<WebsocketMessage> {
        // self.inner.read()
        let message = match self.inner.read() {
            Ok(message) => message,
            Err(err) => {
                return wrap_err!("unable to read message due to err: {}", err);
            },
        };

        Ok(WebsocketMessage::from(message))
    }
    fn send(&mut self, message: String) -> LuaEmptyResult {
        match self.inner.send(Message::Text(message.into())) {
            Ok(_) => Ok(()),
            Err(err) => {
                wrap_err!("unable to send text message due to err: {}", err)
            }
        }
    }
    fn send_bytes(&mut self, message: Vec<u8>) -> LuaEmptyResult {
        match self.inner.send(Message::Binary(message.into())) {
            Ok(_) => Ok(()),
            Err(err) => {
                wrap_err!("unable to send binary message due to err: {}", err)
            }
        }
    }
    fn can_read(&self) -> bool {
        self.inner.can_read()
    }
    fn can_send(&self) -> bool {
        self.inner.can_write()
    }
    fn close(&mut self) -> LuaEmptyResult {
        match self.inner.close(None) {
            Ok(_) => Ok(()),
            Err(err) => {
                wrap_err!("unable to close websocket due to err: {}", err)
            }
        }
    }
    fn get_userdata(self, luau: &Lua) -> LuaValueResult {
        ok_userdata(self, luau)
    }
}

impl LuaUserData for WebsocketWrapper {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "Websocket");
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("readable", |_luau, it, _: LuaValue| {
            Ok(LuaValue::Boolean(it.can_read()))
        });
        methods.add_method("sendable", |_luau, it, _: LuaValue| {
            Ok(LuaValue::Boolean(it.can_send()))
        });
        methods.add_method_mut("read", |luau, it, _: LuaValue| -> LuaValueResult {
            let message = it.read()?;
            message.get_userdata(luau)
        });
        methods.add_method_mut("send", |luau: &Lua, it: &mut WebsocketWrapper, value: LuaValue| {
            let function_name = "Websocket:send(message: string)";
            match value {
                LuaValue::String(s) => it.send(s.to_string_lossy()),
                LuaValue::Table(t) => {
                    match std_json::encode(luau, t, std_json::EncodeOptions { pretty: false, sorted: false }) {
                        Ok(message) => {
                            it.send(message)
                        },
                        Err(err) => {
                            wrap_err!("{}: unable to encode your message to json due to err: {}", function_name, err)
                        }
                    }
                },
                LuaValue::Buffer(buffy) => {
                    it.send_bytes(buffy.to_vec())
                }
                other => {
                    wrap_err!("{}: expected message to be a valid utf-8 string, got: {:?}", function_name, other)
                }
            }
        });
        methods.add_method_mut("close", |_luau, it, _: LuaValue| {
            it.close()
        });
    }
}

fn websocket_connect(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "socket.connect(url: string)";

    let url = match multivalue.pop_front() {
        Some(LuaValue::String(url)) => url.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} called without required argument url (expected string)", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected url to be a string, got: {:?}", function_name, other);
        }
    };

    let url = match Url::parse(&url) {
        Ok(url) => url,
        Err(err) => {
            return wrap_err!("{}: unable to parse url '{}' due to err: {}", function_name, &url, err);
        }
    };

    let (websocket, _response) = match tungstenite::connect(url.as_str()) {
        Ok(pair) => pair,
        Err(err) => {
            return wrap_err!("{}: unable to connect to websocket at '{}' due to err: {}", function_name, &url, err);
        }
    };

    WebsocketWrapper::new(websocket).get_userdata(luau)
}

// use std::net::TcpListener;
// fn socket_host(luau: &Lua, value: LuaValue) -> LuaValueResult {
//     let function_name = "socket.host(url: string)";
//     let url = match value {
//         LuaValue::String(url) => url.to_string_lossy(),
//         other => {
//             return wrap_err!("{}: expected url to be a string, got: {:?}", function_name, other);
//         }
//     };

//     let server = match TcpListener::bind(&url) {
//         Ok(server) => server,
//         Err(err) => {
//             return wrap_err!("{}: unable to bind to socket address '{}' due to err: {}", function_name, &url, err);
//         }
//     };

//     for stream in server.incoming() {
//         let stream = match stream {
//             Ok(s) => s,
//             Err(err) => {
//                 return wrap_err!("{}: tcp stream event came in with an error: {}", function_name, err);
//             }
//         };

//         let mut websocket = match tungstenite::accept(stream) {
//             Ok(websocket) => websocket,
//             Err(err) => {
//                 return wrap_err!("{}: unable to accept websocket due to err: {}", function_name, err);
//             }
//         };

//         let mut i = 0;

//         loop {
//             let message = match websocket.read() {
//                 Ok(message) => message,
//                 Err(err) => {
//                     eprintln!("{}: reading message failed due to err: {}", function_name, err);
//                     continue;
//                 }
//             };

//             println!("message {}", message);

//             i += 1;
//             if i == 100 {
//                 break;
//             }
//         }
//     }

//     todo!()
// }

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("connect", websocket_connect)?
        .build_readonly()
}