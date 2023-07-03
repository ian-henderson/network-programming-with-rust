extern crate ftp;
use ftp::{FtpError, FtpStream};
use std::{io::Cursor, str};

fn run_ftp(addr: &str, user: &str, pass: &str) -> Result<(), FtpError> {
    let mut ftp_stream = FtpStream::connect((addr, 21))?;
    ftp_stream.login(user, pass)?;
    println!("current dir: {}", ftp_stream.pwd()?);

    let data = "A random string to write to a file";
    let mut reader = Cursor::new(data);
    ftp_stream.put("my_file.txt", &mut reader)?;

    ftp_stream.quit()
}

fn main() {
    run_ftp("ftp.dlptest.com", "dlpuser", "rNrKYTX9g7z3RgJRmxWuGHbeu").unwrap();
}
