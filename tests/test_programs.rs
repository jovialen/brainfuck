use brainfuck_interpreter::interpreter::brainfuck;
use brainfuck_lexer::lex;

#[test]
fn hello_world() {
    let src = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string();
    let bf = lex(src);

    assert!(bf.is_ok());

    let mut buf = Vec::new();
    let res = brainfuck(bf.unwrap(), &mut buf);
    assert!(res.is_ok());

    let str: String = buf.into_iter().map(|v| v as char).collect();
    assert_eq!(str, "Hello World!\n".to_string());
}
