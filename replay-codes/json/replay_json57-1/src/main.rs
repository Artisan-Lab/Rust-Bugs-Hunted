extern crate json;

fn main() {
    let capacity = 673957684733028;
    let _ = json::object::Object::with_capacity(capacity);
}

//other data may cause panic:
//1816697882090483224
//3248809668288845358

