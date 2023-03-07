use std::{borrow::Cow, collections::HashMap};

use reqwest::header::CONTENT_TYPE;
use wry::{
    application::{
        dpi::LogicalSize,
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::run_return::EventLoopExtRunReturn,
        window::WindowBuilder,
    },
    http::Response,
    webview::WebViewBuilder,
};

pub fn ticket(url: &str) -> Option<String> {
    #[derive(Debug)]
    enum UserEvents {
        CloseWindow(String),
    }
    let script = include_str!("./script.js");
    let mut ticket = None;
    let mut event_loop = EventLoop::<UserEvents>::with_user_event();
    let proxy = event_loop.create_proxy();
    let ipcproxy = proxy.clone();
    let mut windows = HashMap::new();
    let window = WindowBuilder::new()
        .with_title("滑")
        .with_inner_size(LogicalSize {
            width: 455,
            height: 390,
        })
        .build(&event_loop)
        .unwrap();
    let windowid = window.id();
    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_devtools(true)
        .with_ipc_handler(move |_, s| {
            println!("{}", s);
            let _ = ipcproxy.send_event(UserEvents::CloseWindow(s));
        })
        .with_custom_protocol("ricq".into(), move |request| {
            println!("{:?}", request);
            let _ticket = String::from_utf8_lossy(request.body()).to_string();
            let _ = proxy.send_event(UserEvents::CloseWindow(_ticket));
            Response::builder()
                .header(CONTENT_TYPE, "application/json")
                .body(Cow::from(b"{}".to_vec()))
                .map_err(Into::into)
        })
        .with_initialization_script(script)
        .with_url(url)
        .unwrap()
        .build()
        .unwrap();
    windows.insert(windowid, _webview);
    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("启动滑块窗口"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::UserEvent(UserEvents::CloseWindow(x)) => {
                windows.remove(&windowid);
                ticket = Some(x);
                *control_flow = ControlFlow::Exit
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(x),
                ..
            } => {
                println!("{x:?}");
            }
            _ => (),
        }
    });
    ticket
}
