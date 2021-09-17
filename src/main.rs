#[derive(Debug)]
enum Value {
    Array(Vec<Value>),
    Data(Vec<u8>),
}

fn main() {
    let data = Value::Data(vec![65]);
    let array = Value::Array(vec![
        Value::Data(vec![65]),
        Value::Data(vec![66]),
        Value::Data(vec![67]),
    ]);
    let array_of_arrays = Value::Array(vec![
        Value::Array(vec![
            Value::Data(vec![65]),
            Value::Data(vec![66]),
            Value::Data(vec![67]),
        ]),
        Value::Array(vec![
            Value::Data(vec![68]),
            Value::Data(vec![69]),
            Value::Data(vec![70]),
        ]),
        Value::Array(vec![
            Value::Data(vec![71]),
            Value::Data(vec![72]),
            Value::Data(vec![73]),
        ]),
    ]);
    dbg!(data_to_string(data));
    dbg!(array_to_string(array));
    dbg!(array_to_string(array_of_arrays));
}

fn data_to_string(value: Value) -> String {
    if let Value::Data(bytes) = value {
        if let Ok(string) = String::from_utf8(bytes) {
            return string;
        }
    }
    String::from("")
}

fn array_to_string(value: Value) -> String {
    if let Value::Array(array) = value {
        let mut string = String::new();
        for value in array {
            if let Value::Data(_) = value {
                string = string + &data_to_string(value);
            } else {
                string = string + &array_to_string(value);
            }
        }
        return string;
    }
    String::from("")
}
