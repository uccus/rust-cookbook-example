use crossbeam::channel;
use std::{thread, time};

pub fn test(){
    let (send, recv) = channel::unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s|{
        println!("scope {:?}", thread::current().id());
        s.spawn(|_|{
            for i in 0..n_msgs {
                println!("scope2 {:?}", thread::current().id());
                thread::sleep(time::Duration::from_millis(500));
                send.send(i).unwrap();
            }
        });
    }).unwrap();
    
    
    println!("main {:?}", thread::current().id());

    drop(send);
    for msg in recv.iter(){
        println!("recv: {}", msg);
    }
}