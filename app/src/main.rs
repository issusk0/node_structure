mod utils;
use utils::{ask_data, Node};
use std::thread;
use std::sync::{Arc, Mutex};
//Arc<Mutex<Dato>> para manejar un mismo dato con varios hilos distintos controlando el flujo y integridad de dato * hilo
fn main() {
    let shared_data: Arc<Mutex<Vec<Box<Node>>>> = Arc::new(Mutex::new(Vec::new()));
    let producer_nodes= Arc::clone(&shared_data);

    let producer = thread::spawn(move || {
        loop{
            let quit = utils::ask_data(&producer_nodes);
            if quit {
                break;
            }
        }
    });
    producer.join().unwrap();
    let nodes_guards= shared_data.lock().unwrap();
    println!("Datos almacenados");
    for(i, nodes) in nodes_guards.iter().enumerate(){
        println!("Nodo{}=>{}", i, nodes.data);
    };

}
