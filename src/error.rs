use logos::Span;

struct ErrInfo {
    line_num: usize,
    line: String,
    carat: String
}

fn err_format(source: &str, span: &Span) -> ErrInfo {
    let mut start = 0usize;
    let mut end = 0usize;

    let mut line_num = 1usize;

    for (i, c) in source.chars().enumerate() {
        end = i + 1;
        if c == '\n' {
            if i < span.start {
                line_num += 1;
                start = i + 1;
            }
            if i >= span.end {end = i; break;}
        }
    }

    let line = (&source[start..end]).to_owned();

    let mut carat = String::new();

    for i in start..end {
        if i >= span.start && i < span.end
        {carat.push('^')}
        else {carat.push(' ')}
    }

    return ErrInfo {line_num, line, carat};
}

pub fn error<S: std::fmt::Display>(info: (&str, &Span), msg: S) -> ! {
    let info = err_format(info.0, info.1);
    
    println!(
        "\u{001b}[31mError at line {}:\u{001b}[0m {}\n{}\n\u{001b}[31m{}\u{001b}[0m",
        info.line_num,
        msg,
        info.line,
        info.carat
    );

    std::process::exit(-1);
}

pub fn warning<S: std::fmt::Display>(info: (&str, &Span), msg: S) {
    let info = err_format(info.0, info.1);
    
    println!(
        "\u{001b}[33mWarning at line {}:\u{001b}[0m {}\n{}\n\u{001b}[33m{}\u{001b}[0m",
        info.line_num,
        msg,
        info.line,
        info.carat
    );
}

macro_rules! error_here {
    ($lex:expr, $msg:expr) => {
        error(($lex.source(), &$lex.span()), $msg)
    };
    ($lex:expr, $($msg:expr),+) => {
        error(($lex.source(), &$lex.span()), format!($($msg),+))
    }
}

pub(crate) use error_here;
