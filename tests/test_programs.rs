use std::io::Cursor;

use brainfuck_interpreter::interpreter::interpret;
use brainfuck_lexer::lex;

#[test]
fn hello_world() {
    let src = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string();
    let bf = lex(src);

    assert!(bf.is_ok());

    let mut buf = Vec::new();
    let mut input = Cursor::new(vec![0u8]);
    let res = interpret(&bf.unwrap(), &mut input, &mut buf);
    assert!(res.is_ok());

    let str: String = buf.into_iter().map(|v| v as char).collect();
    assert_eq!(str, "Hello World!\n".to_string());
}

#[test]
fn cat_char() {
    let src = ",.".to_string();
    let bf = lex(src);

    assert!(bf.is_ok());

    let mut buf = Vec::new();
    let mut input = Cursor::new(vec![b'A']);
    let res = interpret(&bf.unwrap(), &mut input, &mut buf);
    assert!(res.is_ok());

    let str: String = buf.into_iter().map(|v| v as char).collect();
    assert_eq!(str, "A".to_string());
}

#[test]
fn cat_string() {
    let src = ",[.,]".to_string();
    let bf = lex(src);

    assert!(bf.is_ok());

    let mut buf = Vec::new();
    let mut input = Cursor::new("This is the way".as_bytes());
    let res = interpret(&bf.unwrap(), &mut input, &mut buf);
    assert!(res.is_ok());

    let str: String = buf.into_iter().map(|v| v as char).collect();
    assert_eq!(str, "This is the way".to_string());
}
