#![feature(convert)]

extern crate temple;

use temple::*;

fn main(){
    let html_string = html(title("test".to_string()),
            h1("title", "Hello!".to_string()) +
            div("container",
                p("", "more stuff".to_string()) +
                p("", "even more stuff".to_string()).as_str()
            ).as_str()
        );

    println!("{}", html_string);
}

