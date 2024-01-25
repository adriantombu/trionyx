use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{PageLoadEvent, WebViewBuilder, WebViewExtMacOS};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let builder = WebViewBuilder::new(&window)
        .with_url("https://blog.otso.fr")?
        // .with_visible(false)
        .with_autoplay(false)
        .with_incognito(true)
        .with_devtools(true)
        .with_on_page_load_handler(move |event, url| {
            if let PageLoadEvent::Finished = event {
                my_page_load_finished_handler(event, url)
            }
        })
        .build()?;

    let js = r#"
        window.addEventListener('DOMContentLoaded', () => {
            console.log("dom loaded");
            return { "html": document.documentElement.innerHTML };
        });
    "#;

    builder.evaluate_script_with_callback(js, |data: String| {
        println!("JS result:");
        println!("{}", data);
    })?;

    // let mut loops = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // dbg!(&event);
        // loops += 1;
        // if loops > 30 {
        //     std::process::exit(0);
        // }

        if let PageLoadEvent::Finished = event {}

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}

fn my_page_load_finished_handler(event: PageLoadEvent, url: String) {
    println!("Page loaded: {}", url);
}
