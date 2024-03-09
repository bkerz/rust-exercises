//Peluqueria no swe que
//requirements:
// - Tiempo necesario para cortar el pelo de Sir Patrick: de 0 a 400 milisegundos (¡que velocidad!)
// - Tiempo necesario para cortar el pelo de los aprendices: de 0 a 600 milisegundos (¡que velocidad!)
// - Tiempo necesario para cobrar en la caja: de 0 a 150 milisegundos
// - Número de clientes para el problema: 50 clientes
// - Tiempo de llegada de los clientes: Deberás decidirlo tú a base de hacer experimentos. Tendrá que ser
// - lo bastante bajo como para poner a prueba las capacidades de la peluquería.

use rand::Rng;
use std::fmt;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self};
use threadpool::ThreadPool; // Rng trait must be in scope to use gen_range
mod threadpool;

fn main() {
    // let (tx, rx) = mpsc::channel::<(Option<Barbers>, i32)>();
    let (tx, rx) = mpsc::channel::<()>();
    let (done_tx, done_rx) = mpsc::channel::<()>();
    let tx = Arc::new(Mutex::new(tx));
    // let mut rand_thread = rand::thread_rng();
    // let mut barbers: Vec<Barbers> = vec![];
    // barbers.push(Barbers::Patrick);
    // barbers.push(Barbers::Student);
    // barbers.push(Barbers::Student);
    let clients = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let threadpool = ThreadPool::new();

    // barbers.sort_by(|_, _| {
    //     let num = rand_thread.gen_range(0..10);
    //     num.cmp(&5)
    // });

    //TODO: first improvement:
    //barber_seats = 3;
    //couch = 5;
    //bar_people_limit = 15;
    // let mut barber_seats = vec![];

    for client in clients.clone() {
        threadpool.execute(client);
    }
    let done_tx_clone = done_tx.clone();
    thread::spawn(move || {
        for _ in clients {
            rx.recv().unwrap();
        }
        done_tx_clone.send(()).unwrap();
    });

    done_rx.recv().unwrap();

    // for i in 0..3 {
    //     let new_tx = tx.clone();
    //     let new_barber = barbers[i].clone();
    //     println!("barber inserted {}", new_barber);
    //     let index = get_barber_index(&new_barber, &barbers);
    //     barber_seats.push(thread::spawn(move || {
    //         barber_work(new_barber, index, new_tx)
    //     }));
    // }
    // for seat in barber_seats {
    //     seat.join().unwrap();
    // }

    // for receiver in rx {
    //     let (maybe_barber, index) = receiver;
    //     match maybe_barber {
    //         Some(barber) => {
    //             println!("A barber is available!");
    //             barbers.push(barber);
    //         }
    //         None => {
    //             barbers.remove(index as usize);
    //         }
    //     }
    // }
}
