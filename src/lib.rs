pub fn elem(elem_type:&str, class:&str, content:String) -> String{
    format!("<{e_type} {class}>{value}</{e_type}>",e_type=elem_type,value=content,class=class)
}

#[test]
fn test_elem(){
    let elem_string = elem("div", "class=\"test\"", String::from_str("content"));
    assert!(elem_string == String::from_str("<div class=\"test\">content</div>"))
}


pub fn html(title:&str, content:String) -> String{
    format!("<!DOCTYPE html><head><title>{}</title></head><body>{}</body></html>", title, content)
}

#[test]
fn test_html(){
    let html_string = html("hello", String::from_str("content"));
    assert!(html_string == String::from_str("<!DOCTYPE html><head><title>hello</title></head><body>content</body></html>"))
}

#[test]
fn test_html_elem(){
    let html_string = html("hello", elem("div", "class=\"test\"", String::from_str("content")));
    assert!(html_string == String::from_str("<!DOCTYPE html><head><title>hello</title></head><body><div class=\"test\">content</div></body></html>"))
}

#[test]
fn test_elem_addition(){
    let html_string = elem("b", "", "content".to_string()) + elem("b", "", "content".to_string()).as_slice();
    assert!(html_string == "<b >content</b><b >content</b>")
}

pub fn h1(class:&str, content:String) -> String{ elem("h1", class, content) }
pub fn div(class:&str, content:String) -> String{ elem("div", class, content) }
pub fn span(class:&str, content:String) -> String{ elem("span", class, content) }
pub fn img(class:&str, content:String) -> String{ elem("img", class, content) }
pub fn p(class:&str, content:String) -> String{ elem("p", class, content) }
pub fn ul(class:&str, content:String) -> String{ elem("ul", class, content) }
pub fn li(class:&str, content:String) -> String{ elem("li", class, content) }

#[test]
fn test_div(){
    let div_string = div("", String::from_str("content"));
    assert!(div_string == String::from_str("<div >content</div>"))
}

#[test]
fn test_h1(){
    let h1_string = h1("", String::from_str("content"));
    assert!(h1_string == String::from_str("<h1 >content</h1>"))
}
