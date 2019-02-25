use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use web_server::ThreadPool;
use std::sync::{Arc, Mutex, mpsc};
/*use std::sync::Mutex;
use std::sync::mpsc;*/
use rand::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let buffr = Arc::new(Mutex::new([0x0; 64]));
    let counter = Arc::new(Mutex::new(0));
    let pool = ThreadPool::new(64);
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let sender = Arc::new(Mutex::new(sender.clone()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let sender = Arc::clone(&sender);
        let receiver = Arc::clone(&receiver);
        let counter = Arc::clone(&counter);
        let buffr = Arc::clone(&buffr);
        pool.execute(move || {
            handle_connection(stream, counter, buffr, sender, receiver);
        });
    }
}


pub fn handle_connection(mut stream: TcpStream, counter: Arc<Mutex<i32>>, buf: Arc<Mutex<[u64; 64]>>, sen: Arc<Mutex<mpsc::Sender<u64>>>, rec: Arc<Mutex<mpsc::Receiver<u64>>>) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let rand_u64: u64 = rand::thread_rng().gen();
    let key = rand_u64;
    let com = 0;
    let mut cpt = counter.lock().unwrap();
    let com = *cpt;
    let mut buff = buf.lock().unwrap();
    buff[com as usize] = key;
    println!("random key generated {} counter {}", key, com);
    println!("value dans le buff {} ", buff[com as usize]);
    std::mem::drop(buff);
    *cpt += 1;
    std::mem::drop(cpt);

    if com == 63 {
        let buff = buf.lock().unwrap();
        let mut result = key;
        println!("VALEEUR DE LA KLEY {} ", result);

        for i in 0..64 {
            result ^= buff[i];
            println!("dans la boucle {}", result);
        }

        let res: String = result.to_string();
        println!("Result calculated {} ", result);
        for _ in 0..64 {
            let sender = sen.lock().unwrap().send(result).unwrap();
        }
        stream.write(res.as_bytes()).unwrap();
    } else {
        let received = rec.lock().unwrap().recv().unwrap();
        let result: String = received.to_string();
        stream.write(result.as_bytes()).unwrap();
        println!("value received {} ", received);
    }
    println!("goodbye n°{} ", com);
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
