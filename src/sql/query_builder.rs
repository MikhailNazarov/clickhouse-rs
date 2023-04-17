use std::fmt::{Display, Formatter, Pointer, Result, Write};
use chrono::{DateTime, Utc};

pub struct QueryBuilder{
    query: String
}

pub trait SqlLiteral{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}


impl QueryBuilder{
    pub fn new(init: impl Into<String>)->Self{
        let init = init.into();

        Self{
            query: init
        }
    }

    pub fn build(self)-> String {
        self.query
    }

    pub fn push(&mut self, sql: impl Display)->&mut Self{
        write!(self.query, "{}", sql).expect("error formatting `sql`");
        self
    }


    pub fn push_bind<T:SqlLiteral>(&mut self, value: T)-> &mut Self{
        write!(self.query, "{}", Wrapper{
            value
        }).expect("error formatting `sql`");
        self
    }

    pub fn separated<Sep: Display>(&mut self, separator: Sep) -> Separated<Sep> {
        Separated::new(self, separator)
    }
}

pub struct Separated<'qb, Sep: Display>{
    push_separator: bool,
    separator: Sep,
    qb: &'qb mut QueryBuilder
}

impl<'qb, Sep: Display> Separated<'qb, Sep>{
    fn new(qb: &'qb mut QueryBuilder, sep: Sep)-> Self{
        Self{
            push_separator: false,
            separator: sep,
            qb
        }
    }

    pub fn push_unseparated(&mut self, sql: impl Display) -> &mut Self {
        self.qb.push(sql);
        self
    }


    pub fn push(&mut self, sql: impl Display)->&mut Self{
        if self.push_separator {
            self.qb
                .push(format_args!("{}{}", self.separator, sql));
        } else {
            self.qb.push(sql);
            self.push_separator = true;
        }

        self
    }


    pub fn push_bind<T:SqlLiteral>(&mut self, value: T)-> &mut Self{
        if self.push_separator {
            self.qb.push(&self.separator);
        }

        self.qb.push_bind(value);
        self.push_separator = true;
        self
    }
}

struct Wrapper<T: SqlLiteral>{
    value: T
}


struct WrapperRef<'a, T: SqlLiteral>{
    value: &'a T
}


impl<T: SqlLiteral> Display for Wrapper<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.value.fmt(f)
    }
}

impl<'a, T: SqlLiteral> Display for WrapperRef<'a, T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.value.fmt(f)
    }
}

impl SqlLiteral for &str{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("'{}'", escape_string(self)))
    }
}

impl SqlLiteral for String{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        SqlLiteral::fmt(&self.as_str(), f)
    }
}



impl<T: SqlLiteral> SqlLiteral for Vec<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("[")?;

        let mut push_separator = false;
        for value in self{
            if push_separator{
                f.write_str(", ")?;
            } else {
                push_separator = true;
            }
            f.write_fmt(format_args!("{}", WrapperRef{
                value
            }))?;
        }

        f.write_str("]")
    }
}

impl SqlLiteral for i32{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl SqlLiteral for u32{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl SqlLiteral for i16{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl SqlLiteral for u16{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl SqlLiteral for i8{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl SqlLiteral for u8{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}


impl SqlLiteral for f32{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl SqlLiteral for f64{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}



fn escape_string(s: &str)->String{
    let mut r = String::with_capacity(s.len());
    for c in s.chars(){
        match c{
            '\'' => r+= "\\'",
            '\\' => r+= "\\\\",
            x => r.push(x)
        };
    }
    r
}

#[cfg(test)]
mod tests{
    use crate::sql::query_builder::escape_string;
    use crate::sql::QueryBuilder;

    #[test]
    fn test_escape_string() {
        let actual = escape_string(r#"foo b'ar\\"#);
        assert_eq!(actual, r#"foo b\'ar\\\\"#);
    }

    #[test]
    fn test_push() {
        let mut qb = QueryBuilder::new("SELECT * FROM test");
        qb.push(" WHERE foo = 'bar'");
        assert_eq!(qb.build(), "SELECT * FROM test WHERE foo = 'bar'");

    }

    #[test]
    fn test_push_bind_i32() {
        let mut qb = QueryBuilder::new("SELECT * FROM test");
        qb.push(" WHERE foo = ").push_bind(123);
        assert_eq!(qb.build(), "SELECT * FROM test WHERE foo = 123");

    }

    #[test]
    fn test_push_bind_u32() {
        let mut qb = QueryBuilder::new("SELECT * FROM test");
        qb.push(" WHERE foo = ").push_bind(123u32);
        assert_eq!(qb.build(), "SELECT * FROM test WHERE foo = 123");

    }

    #[test]
    fn test_push_bind_f32() {
        let mut qb = QueryBuilder::new("SELECT * FROM test");
        qb.push(" WHERE foo = ").push_bind(222.33f32);
        assert_eq!(qb.build(), "SELECT * FROM test WHERE foo = 222.33");

    }

    #[test]
    fn test_push_bind_str () {
        let mut qb = QueryBuilder::new("SELECT * FROM test");
        qb.push(" WHERE foo = ").push_bind("bar");
        assert_eq!(qb.build(), "SELECT * FROM test WHERE foo = \'bar\'");

    }

    #[test]
    fn test_push_bind_string () {
        let mut qb = QueryBuilder::new("SELECT * FROM test");
        qb.push(" WHERE foo = ").push_bind("bar".to_string());
        assert_eq!(qb.build(), "SELECT * FROM test WHERE foo = \'bar\'");

    }

    #[test]
    fn test_push_bind_vec_int () {
        let mut qb = QueryBuilder::new("SELECT * FROM test");
        qb.push(" WHERE foo IN ").push_bind(vec![1,2,3]);
        assert_eq!(qb.build(), "SELECT * FROM test WHERE foo IN [1, 2, 3]");

    }
}