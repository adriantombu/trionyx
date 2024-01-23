use headless_chrome::{Browser, LaunchOptions};
use std::error::Error;
use std::ffi::OsStr;
use std::thread;

// TODO: try to replace later with a lighter alternative (WebDriver, wry, ... ?)
fn get_html() -> Result<String, Box<dyn Error>> {
    let options = LaunchOptions {
        headless: false,
        window_size: Some((1920, 1080)),
        args: vec![OsStr::new("--disable-notifications")],
        ..Default::default()
    };
    let browser = Browser::new(options)?;

    let tab = browser.new_tab()?;
    tab.navigate_to("https://blog.otso.fr")?;
    tab.wait_until_navigated()?;
    thread::sleep(std::time::Duration::from_millis(300));

    let content = tab.get_content()?;

    Ok(content)
}
