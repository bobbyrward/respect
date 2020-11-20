use crate::RespectError;
use crate::Value;

/// Encode value as a string
pub(crate) fn resp_encode(value: &Value) -> Result<Vec<u8>, RespectError> {
    Ok(match value {
        Value::SimpleString(s) => {
            if s.contains('\n') {
                return Err(RespectError::unknown(&format!(
                    "Value is not binary safe: {}",
                    s
                )));
            }

            format!("+{}\r\n", s).as_bytes().iter().copied().collect()
        }
        Value::Error(s) => {
            if s.contains('\n') {
                return Err(RespectError::unknown(&format!(
                    "Value is not binary safe: {}",
                    s
                )));
            }

            format!("-{}\r\n", s).as_bytes().iter().copied().collect()
        }
        Value::Integer(i) => format!(":{}\r\n", i).as_bytes().iter().copied().collect(),
        Value::BulkString(bytes) => {
            let mut encoded = Vec::new();
            encoded.extend_from_slice(format!("${}\r\n", bytes.len()).as_bytes());
            encoded.extend_from_slice(bytes);
            encoded.extend_from_slice(b"\r\n");
            encoded
        }
        Value::Array(values) => {
            let mut encoded = Vec::new();
            encoded.extend_from_slice(format!("*{}\r\n", values.len()).as_bytes());

            for item in values.iter() {
                encoded.extend_from_slice(&resp_encode(item)?);
            }

            encoded
        }
        Value::Null => String::from("*-1\r\n").as_bytes().iter().copied().collect(),
    })
}
