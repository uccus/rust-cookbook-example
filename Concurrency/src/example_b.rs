use std::thread;
use crossbeam::channel::bounded;

pub fn test(){
    let (s1, r1) = bounded(1);
    let (s2, r2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;
    crossbeam::scope(|s|{
        s.spawn(|_|{
            for i in 0..n_msgs {
                s1.send(i).unwrap();
                println!("A通道发送了一条数据: {}", i);
            }
            drop(s1);
        });
        
        for _ in 0..n_workers {
            let (ss2, r1) = (s2.clone(), r1.clone());
            s.spawn(move |_|{
                for msg in r1.iter() {
                    println!("工作线程{:?}接收到数据{}", thread::current().id(), msg);
                    ss2.send(msg).unwrap();
                }
            });
        }

        drop(s2);

        for msg in r2.iter(){
            println!("主线程接收到数据{}", msg);
        }
    }).unwrap();
    
    
}