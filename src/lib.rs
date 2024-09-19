use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;      //Job, sería básicamente la funcion a implementar (para ello debe estar dentro de un Box,
//para que funcione como un trait object )
struct Worker {
    id: usize,
    thread: thread::JoinHandle<Arc<Mutex<Receiver<Job>>>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            //implantacion del receiver dentro del thread, por medio del cual se recibirá la funcion a implementar
            loop {
                let job = receiver.lock().unwrap().recv().unwrap(); //Especificar que se hizo del candado
                //receiver es la parte receptora del canal, recv es la recepcion de lo enviado
                println!("Worker {} got a job; executing.", id);
                // Mover el `job` fuera del Box y ejecutarlo
                job();        //No necesitamos desreferenciar el Box, sino simplemente llamarlo
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver_ptr = Arc::new(Mutex::new(receiver)); //Para poder compartir una referencia del "receiver" a cada worker se utiliza "Arc"

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver_ptr)));   //Se clona el puntero al receiver del canal
            // create some workers and store them in the vector
        }

        ThreadPool {
            workers,
            sender,     //Se almacena el sender del canal
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap(); //Puede hacer uso del "sender", porque ya lo tenía almacenado en su estructura
    }                                   //con eso le envía al worker por el canal la funcion a implementar en este caso "handle_client"
}

/* Eliminación del *job: En lugar de intentar desreferenciar el Box con (*job)(), simplemente
    llamamos a job() directamente. Esto mueve el Box y ejecuta la closure que contiene.En Rust, los
    Box<dyn FnOnce()> implementan FnOnce, por lo que puedes llamarlos directamente sin desreferenciarlos.
*/

/*
* Puntero de referencia mutable al extremo receptor del canal
* Quitar un job (trabajo) de la cola del canal implica mutar el receiver, por lo que los hilos necesitan
    una forma segura de compartir y modificar receiver.
* Recuerde los thread-safe smart pointers analizados en el Capítulo 16: para compartir la propiedad
    entre varios threads y permitir que los threads muten el valor, necesitamos usar Arc<Mutex<T>>
* El tipo Arc permitirá que varios workers posean el receptor, y Mutex garantizará que solo un worker
    obtenga un trabajo (job) del receptor a la vez
 */



/*ThreadPool: Es la estructura que manejará el pool de hilos.
* new: Crea una instancia de ThreadPool con un tamaño específico. La función assert!(size > 0);
    asegura que el pool tenga al menos un hilo.
* execute: Se utiliza para ejecutar una función (closure) en uno de los hilos del pool.
* El tipo de parámetro F es una función que se ejecutará una sola vez (FnOnce),
    es segura para ser enviada entre hilos (Send), y tiene una vida estática ('static), es decir,
    no tiene referencias temporales.*/

/*El parámetro de tipo F también tiene el trait bound Send y el lifetime bound 'static, que son
    útiles en nuestra situación: necesitamos Send para transferir el closure de un hilo a otro y 'static
    porque no sabemos cuánto tardará el hilo en ejecutarse. Vamos a crear un method execute en ThreadPool
    que tomará un parámetro genérico de tipo F con estos límites*/

/*Todavía usamos el () después de FnOnce porque este FnOnce representa un closure que no toma
    parámetros y no devuelve un valor. Al igual que las definiciones de función, el tipo de devolución
    puede omitirse de la firma, pero incluso si no tenemos parámetros, aún necesitamos los paréntesis.*/