use std::fs::File;
use std::fs;
use std::io::{BufReader, Read, Write};
use std::net::{Shutdown, TcpListener};

fn read_entries_in_dir(path: &str) {
  let mut directory = fs::read_dir(path)
    .expect("There was an error reading the contents of the directory")
    .into_iter();

  while let Some(Ok(file)) = directory.next() {

    println!("{:?}", file.file_name());

  }

}

fn create_file_in_dir(path: &str, file_name: &str) {

  File::create(path.to_owned() +"/" + file_name)
    .expect("There was an error creating the specified file");

}

fn send_file_as_stream(file_name: &str, listener: &TcpListener) -> () {
 
  // let (mut stream, _addr) = listener.accept()?
  for stream in listener.incoming() {

    let file = File::open(file_name)
      .expect("There was a problem");

    let mut buffered_reader = BufReader::new(file);
    let mut read_contents = vec![];

    buffered_reader.read_to_end(&mut read_contents)
    .expect("There was a problem");

    let mut stream = stream
      .expect("lol");

    println!("{:?}", &read_contents);

    let number_of_bytes_written = &stream
      .write(&read_contents)
      .expect("There was an error sending the bytes");

    println!("{number_of_bytes_written}");

    stream.read(&mut read_contents)
      .expect("lol");


    read_contents.make_ascii_lowercase();

    println!("{:?}", read_contents);

  };

  // stream.write(b"Hi!!!")?;

  // println!("{:?}", stream);

  // stream.flush()?;

  // println!("{:?}", stream);

  // stream.shutdown(Shutdown::Both)

}



fn main() {

  let listener = TcpListener::bind("localhost:8080")
    .expect("There was an error binding to the specified address");

  send_file_as_stream("C:\\Users\\User\\Desktop\\Textbooks\\samuel\\nevin.txt", &listener)
    // .expect("There was a problem :P");


  
}
