extern crate clipboard;
use clipboard::ClipboardContext;

fn main() {
    let mut cc = ClipboardContext::new().unwrap();
    println!("Current clipboard contents: {:?}", cc.get_contents());
    let s = "hello";
    println!("Setting clipboard contents to \"{}\".", s);
    cc.set_contents(s.to_owned()).unwrap();
    if cfg!(target_os="linux") {
        loop {}
    }
}
