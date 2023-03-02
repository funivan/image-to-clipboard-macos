use std::env;
use std::process::exit;

use cocoa::appkit::{NSImage, NSPasteboard};
use cocoa::base::nil;
use cocoa::foundation::{NSArray, NSString};

fn main() {
    let path = env::args().into_iter().skip(1).next();
    match path {
        Some(String::from("-h")) => {
            help();
            exit(0);
        }
        Some(path) => {
            if !std::path::Path::new(&path).exists() {
                println!("File {} does not exist", path);
                exit(2);
            }
            unsafe {
                let image = NSImage::alloc(nil).initWithContentsOfFile_(
                    NSString::alloc(nil).init_str(&path)
                );
                let pasteboard = NSPasteboard::generalPasteboard(nil);
                pasteboard.clearContents();
                pasteboard.writeObjects(NSArray::arrayWithObject(nil, image));
            }
            help();
            exit(0)
        }
        _ => {
            println!("You need to provide a file path as first argument");
            help();
            exit(1);
        }
    }
}

fn help() {
    println!("Image copied to clipboard");
    println!("image-to-clipboard <path>");
    println!("image-to-clipboard -h - show help");
}
