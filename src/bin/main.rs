extern crate servidor_http_threadpool;
use servidor_http_threadpool::ThreadPool;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::thread::Thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {       //PASANDO UN CLOSURE COMO PARAMETRO
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // --snip--

    let get = b"GET / HTTP/1.1\r\n";    //un CRFL (Carriage Return Feed Line)
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!(
        "{}\r\ncontent-length:{}\r\n\r\n{}", //ES SUMANENTE IMPORTANTE AGREGAR LA PARTE DE "content-length", SINO NO LO RECONOCE EL NAVEGADOR
        status_line,
        contents.len(),
        contents
    );

    println!("{:?}", response); /// //
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // --snip--
}


//SERÍA MUCHO MEJOR MANEJAR EL "for" DEL MAIN DE LA SIGUIENTE MANERA CON UN WHILE LET(manejo de errores):
/*use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    while let Ok(stream) = listener.incoming().next() {
        match stream {
            Ok(stream) => {
                // Aquí manejamos la conexión
                println!("Conexión establecida");
            }
            Err(e) => {
                // Aquí manejamos cualquier error en la conexión
                println!("Error al aceptar conexión: {}", e);
            }
        }
    }

    // Bloque else, se ejecuta si el while let no se cumple más (es decir, si hubo un Err)
    println!("Cerrando el servidor, error al escuchar conexiones.");
}
*/