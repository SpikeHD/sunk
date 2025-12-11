use sunk::Streamable;

pub fn main() {
  let username = "admin";
  let password = "Lego2456";
  let site = "https://stylish-zebra.pikapod.net";

  let client = sunk::Client::new(site, username, password).unwrap();

  // Update the library.
  client.ping().unwrap();
  client.scan_library().unwrap();

  // Fetch some songs and play them.
  let mut random = sunk::song::Song::random(&client, 1).unwrap();
  for mut song in random {
    println!("{:?}", song);

    // Read the bytes in 1024 byte chunks.
    let stream = song.stream(&client);

    match stream {
      Ok(mut reader) => {
        let mut buffer = [0; 1024];
        loop {
          let n = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
              eprintln!("Error reading stream: {}", e);
              break;
            }
          };
          // Here you would typically send the buffer to an audio output device.
          // For this example, we'll just print the number of bytes read.
          println!("Read {} bytes", n);
        }
      }
      Err(e) => {
        eprintln!("Error streaming song: {}", e);
      }
    }
  }
}
