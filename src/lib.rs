
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

pub fn div(class:&str, content:String) -> String{ elem("div", class, content) }
pub fn span(class:&str, content:String) -> String{ elem("span", class, content) }

#[test]
fn test_div(){
    let div_string = div("", String::from_str("content"));
    assert!(div_string == String::from_str("<div >content</div>"))
}
