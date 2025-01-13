use std::env;
use std::io;
use std::io::Read;

use apache_avro::schema::EnumSchema;
use apache_avro::schema::RecordField;
use apache_avro::schema::RecordSchema;
use apache_avro::schema::Schema;

use crate::bind;
use crate::lift;

pub fn num2str(num: usize, symbols: &[String]) -> Result<&String, io::Error> {
    symbols
        .get(num)
        .ok_or_else(|| io::Error::other("invalid number"))
}

pub fn enum2str(num: usize, e: &EnumSchema) -> Result<&String, io::Error> {
    num2str(num, &e.symbols)
}

pub fn fields2str<'a>(
    num: usize,
    name: &str,
    fields: &'a [RecordField],
) -> Result<&'a String, io::Error> {
    let filtered = fields.iter().filter(|&r: &&RecordField| r.name == name);
    let mut mapd = filtered.map(|r: &RecordField| &r.schema);
    let s: &Schema = mapd
        .next()
        .ok_or_else(|| io::Error::other("invalid schema"))?;
    let e: &EnumSchema = match s {
        Schema::Enum(e) => Ok(e),
        _ => Err(io::Error::other("invalid schema")),
    }?;
    enum2str(num, e)
}

pub fn record2str<'a>(
    num: usize,
    name: &str,
    r: &'a RecordSchema,
) -> Result<&'a String, io::Error> {
    let fields: &[RecordField] = &r.fields;
    fields2str(num, name, fields)
}

pub fn schema2str<'a>(
    num: usize,
    name: Option<&str>,
    s: &'a Schema,
) -> Result<&'a String, io::Error> {
    match s {
        Schema::Enum(e) => enum2str(num, e),
        Schema::Record(rs) => match name {
            Some(n) => record2str(num, n, rs),
            None => Err(io::Error::other("colum name missing")),
        },
        _ => Err(io::Error::other("invalid schema")),
    }
}

pub fn schema_string2str2writer<F>(
    num: usize,
    name: Option<&str>,
    schema: &str,
    mut writer: F,
) -> Result<(), io::Error>
where
    F: FnMut(&str) -> Result<(), io::Error>,
{
    let parsed = Schema::parse_str(schema).map_err(io::Error::other)?;
    let s: &str = schema2str(num, name, &parsed)?;
    writer(s)
}

pub fn schema_string2str2stdout(
    num: usize,
    name: Option<&str>,
    schema: &str,
) -> Result<(), io::Error> {
    schema_string2str2writer(num, name, schema, |s: &str| {
        println!("{s}");
        Ok(())
    })
}

pub fn stdin2schema_string2str2stdout(
    num: usize,
    name: Option<&str>,
    schema_size_max: u64,
) -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let mut limited = il.take(schema_size_max);
    let mut buf: String = String::new();
    limited.read_to_string(&mut buf)?;
    schema_string2str2stdout(num, name, &buf)
}

pub struct Input {
    pub num: usize,
    pub name: Option<String>,
    pub schema_size_max: u64,
}

impl Input {
    pub fn stdin2schema_string2str2stdout(&self) -> Result<(), io::Error> {
        let o: Option<&str> = self.name.as_deref();
        stdin2schema_string2str2stdout(self.num, o, self.schema_size_max)
    }
}

pub fn input2stdout(i: Input) -> Result<(), io::Error> {
    i.stdin2schema_string2str2stdout()
}

pub struct RawInput {
    pub num: String,
    pub name: Option<String>,
    pub schema_size_max: String,
}

impl TryFrom<RawInput> for Input {
    type Error = io::Error;
    fn try_from(r: RawInput) -> Result<Self, Self::Error> {
        let num: usize = str::parse(r.num.as_str()).map_err(io::Error::other)?;
        let schema_size_max: u64 =
            str::parse(r.schema_size_max.as_str()).map_err(io::Error::other)?;
        Ok(Self {
            num,
            name: r.name,
            schema_size_max,
        })
    }
}

pub fn raw2input(raw: RawInput) -> Result<Input, io::Error> {
    raw.try_into()
}

pub fn env2raw(
    num_key: &str,
    name_key: &str,
    schema_limit_key: &str,
) -> Result<RawInput, io::Error> {
    let num: String = env::var(num_key).map_err(io::Error::other)?;
    let name: Option<String> = env::var(name_key).map_err(io::Error::other).ok();
    let schema_size_max: String = env::var(schema_limit_key).unwrap_or_else(|_| "1048576".into());
    Ok(RawInput {
        num,
        name,
        schema_size_max,
    })
}

pub fn env2raw_default() -> Result<RawInput, io::Error> {
    env2raw("ENV_ENUM_INDEX", "ENV_ENUM_COLUMN", "ENV_SCHEMA_SIZE_MAX")
}

pub fn env2input_default() -> Result<Input, io::Error> {
    bind!(env2raw_default, lift!(raw2input))()
}

pub fn env2input2stdout_default() -> Result<(), io::Error> {
    bind!(env2input_default, lift!(input2stdout))()
}
