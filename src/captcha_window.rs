use std::collections::HashMap;

use wry::{
    application::{dpi::PhysicalSize, platform::run_return::EventLoopExtRunReturn},
    http::ResponseBuilder,
};

pub fn ticket(url: &str) -> Option<String> {
    #[derive(Debug)]
    enum UserEvents {
        CloseWindow(String),
    }
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };
    let mut ticket = None;
    let mut event_loop = EventLoop::<UserEvents>::with_user_event();
    let proxy = event_loop.create_proxy();
    let mut windows = HashMap::new();
    let window = WindowBuilder::new()
        .with_title("滑")
        .with_inner_size(PhysicalSize {
            width: 455,
            height: 390,
        })
        .build(&event_loop)
        .unwrap();
    let windowid = window.id();
    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_url(url)
        .unwrap()
        .with_devtools(true)
        .with_custom_protocol("ricq".into(), move |request| {
            let _ticket = String::from_utf8_lossy(request.body()).to_string();
            let _ = proxy.send_event(UserEvents::CloseWindow(_ticket));
            ResponseBuilder::new()
                .status(200)
                .body("ok".as_bytes().to_vec())
        })
        .with_initialization_script(
            r#"
        (function() {
            var origOpen = XMLHttpRequest.prototype.open;
            XMLHttpRequest.prototype.open = function() {
                this.addEventListener('load', function() {
                    if (this.responseURL == 'https://t.captcha.qq.com/cap_union_new_verify') {
                        var j = JSON.parse(this.responseText);
                        if (j.errorCode == '0') {
                            console.log(j.ticket);
                            fetch("https://ricq.ticket", {
                                method: "POST",
                                body: j.ticket
                            });
                            window.ipc.postMessage('close');
                        }
                    }
        
                });
                origOpen.apply(this, arguments);
            };
        })();
        "#,
        )
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
                println!("{:?}", x);
            }
            _ => (),
        }
    });
    ticket
}