use std::io;
use std::process::ExitCode;

fn sub() -> Result<(), io::Error> {
    rs_avro_enum_num2str::num2str::env2input2stdout_default()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
