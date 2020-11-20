use respect::Value;

#[test]
fn test_value_to_vec() {
    let tests = [
        (
            "+Hello, World!\r\n",
            Value::SimpleString(String::from("Hello, World!")),
        ),
        (
            "-Error Message\r\n",
            Value::Error(String::from("Error Message")),
        ),
        // NOTE: Null is always encoded as an array instead of a bulkstring.
        // This may end up causing issues.
        ("*-1\r\n", Value::Null),
        ("$1\r\n?\r\n", Value::BulkString(vec![b'?'])),
        (
            "$4\r\nBulk\r\n",
            Value::BulkString(vec![b'B', b'u', b'l', b'k']),
        ),
        (":0\r\n", Value::Integer(0)),
        (":12\r\n", Value::Integer(12)),
        (":-1234\r\n", Value::Integer(-1234)),
        (":9999999\r\n", Value::Integer(9999999)),
        ("*-1\r\n", Value::Null),
        ("*0\r\n", Value::Array(vec![])),
        (
            "*1\r\n+Item 1\r\n",
            Value::Array(vec![Value::SimpleString(String::from("Item 1"))]),
        ),
        (
            "*2\r\n+Item 1\r\n:456\r\n",
            Value::Array(vec![
                Value::SimpleString(String::from("Item 1")),
                Value::Integer(456),
            ]),
        ),
        (
            "*2\r\n+Item 1\r\n:456\r\n",
            Value::Array(vec![
                Value::SimpleString(String::from("Item 1")),
                Value::Integer(456),
            ]),
        ),
        (
            "*3\r\n$6\r\nst\ning\r\n*2\r\n+SubItem1\r\n+SubItem2\r\n-Error\r\n",
            Value::Array(vec![
                Value::BulkString(b"st\ning".to_vec()),
                Value::Array(vec![
                    Value::SimpleString(String::from("SubItem1")),
                    Value::SimpleString(String::from("SubItem2")),
                ]),
                Value::Error(String::from("Error")),
            ]),
        ),
    ];

    for (expected, value) in &tests {
        let encoded = value.to_vec().unwrap();
        assert_eq!(
            &encoded[..],
            expected.as_bytes(),
            "{:?} != {}",
            std::str::from_utf8(&encoded).unwrap(),
            expected
        );
    }
}
