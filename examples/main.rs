#![feature(convert)]

extern crate temple;

use temple::{ html, elem };

fn main(){
    let html_string = html("test",
            elem("h1", "title", "Hello!".to_string()) +
            elem("div", "container",
                elem("p", "", "more stuff".to_string()) +
                elem("p", "", "even more stuff".to_string()).as_str()
            ).as_str()
        );

    println!("{}", html_string);
}

