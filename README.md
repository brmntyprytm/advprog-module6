# **Advanced Programming - Module 6: Concurrency**

## Bramantyo Priyo Utomo - 2206821563

### 1. `handle_connection` method elaboration

- The handle_connection method in the provided Rust code is responsible for handling incoming TCP connections from clients. It reads the HTTP request from the client and prints it to the console.
- `let buf_reader = BufReader::new(&mut stream);` This line creates a buffered reader from the mutable TCP stream. The buffered reader allows efficient reading of data from the stream.
- `let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();` This line reads the HTTP request from the client. The `lines()` method reads the request line by line. The `map()` method transforms each line into a `Result<String, io::Error>`. The `unwrap()` method is used to unwrap the Result and get the String value. The `take_while()` method takes lines from the iterator until a line that is empty is encountered. The `collect()` method collects the lines into a `Vec<String>`.
- `println!("Request: {:#?}", http_request);` This line prints the HTTP request to the console. The {:#?} syntax is used to pretty-print the request.
