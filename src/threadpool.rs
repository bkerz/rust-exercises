use crate::mpsc;
use rand::{self, Rng};
use std::{
    fmt,
    sync::{mpsc::Sender, Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};
#[derive(Copy, Clone, Debug)]
enum Barbers {
    Patrick,
    Student,
}

impl fmt::Display for Barbers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Barbers::Student => "Student",
                Barbers::Patrick => "Patrick",
            }
        )
    }
}

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: mpsc::Sender<i32>,
    barbers: Arc<Mutex<Vec<Barbers>>>,
}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut rand_thread = rand::thread_rng();
        let mut barbers: Vec<Barbers> = Vec::with_capacity(3);
        barbers.push(Barbers::Patrick);
        barbers.push(Barbers::Student);
        barbers.push(Barbers::Student);
        barbers.sort_by(|_, _| {
            let num = rand_thread.gen_range(0..10);
            num.cmp(&5)
        });
        let mut pool = Self {
            workers: Vec::with_capacity(3),
            sender,
            barbers: Arc::new(Mutex::new(barbers)),
        };
        for barber_index in 0..3 {
            let barbers_clone = Arc::clone(&pool.barbers);
            let receiver = Arc::clone(&receiver);
            let handle = thread::spawn(move || {
                loop {
                    // Receive a message from the main thread to indicate work
                    println!("thread {} waiting for work", barber_index);
                    let client: i32 = receiver.lock().unwrap().recv().unwrap();
                    println!("thread {} working on client {}!", barber_index, client);
                    // Do some work here
                    let mut list: MutexGuard<'_, Vec<Barbers>> = barbers_clone.lock().unwrap();
                    barber_work(list[barber_index], &mut list);
                    // Simulate work by sleeping for a short time
                    println!("thread {} finished work on client {}", barber_index, client);

                    // Signal that the work is done
                    // Note: This is not necessary in this example, but can be useful
                    // if you want to know when a worker thread has completed its work.
                    // sender.send(()).unwrap();
                }
            });
            pool.workers.push(handle);
        }

        pool
    }

    pub fn execute(&self, client: i32) {
        println!("executing");
        self.sender.send(client).unwrap();
        println!("work starts");
        for worker in &self.workers {
            worker.thread().unpark();
            break;
        }
    }
}
fn barber_work(barber: Barbers, barbers: &mut MutexGuard<'_, Vec<Barbers>>) {
    let student_time = 600;
    let patrick_time = 400;
    match barber {
        Barbers::Patrick => {
            println!("Patrick working!");
            println!("Patrick is busy right now");
            // tx.send((None, index)).unwrap();
            let index = get_barber_index(&barber, &barbers);
            barbers.remove(index as usize);
            thread::sleep(std::time::Duration::from_millis(patrick_time));
            // tx.send((Some(Barbers::Patrick), index)).unwrap();
            barbers.push(Barbers::Patrick);
            println!("Patrick is now available");
        }
        _ => {
            println!("Student working!");
            println!("Student is busy right now");
            // tx.send((None, index)).unwrap();
            let index = get_barber_index(&barber, &barbers);
            barbers.remove(index as usize);
            thread::sleep(std::time::Duration::from_millis(student_time));
            // tx.send((Some(Barbers::Patrick), index)).unwrap();
            barbers.push(Barbers::Patrick);
            println!("Student is now available");
        }
    }
}

fn get_barber_index(item: &Barbers, list: &Vec<Barbers>) -> i32 {
    match item {
        Barbers::Patrick => list
            .iter()
            .position(|x| match x {
                Barbers::Patrick => true,
                Barbers::Student => false,
            })
            .unwrap() as i32,
        Barbers::Student => list
            .iter()
            .position(|x| match x {
                Barbers::Student => true,
                Barbers::Patrick => false,
            })
            .unwrap() as i32,
    }
}
