use log::*;
use simplelog::CombinedLogger;
use simplelog::TermLogger;
use simplelog::WriteLogger;
use walkdir::WalkDir;
use threadpool::ThreadPool;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{BufReader, Read, Error};
use std::sync::mpsc::channel;
use ring::digest::{Context, Digest, SHA256};

fn compute_digest<P: AsRef<Path>>(path: P) -> Result<(Digest, P), Error> {
    println!("thread: {:?}", std::thread::current().id());
    let mut reader = BufReader::new(File::open(&path)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0;1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        
        context.update(&buffer[..count]);
    }
    
    Ok((context.finish(), path))
}

pub fn test() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, simplelog::Config::default(), File::create("./test.log").unwrap())
    ]).unwrap();
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for entry in WalkDir::new(".\\src")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir()){
            println!("entry: {}", entry.path().to_str().unwrap());
            let path = entry.path().to_owned();
            let tx = tx.clone();
            pool.execute(move || {
                let res = compute_digest(path);
                tx.send(res).unwrap();
            });
    }
    
    drop(tx);
    
    for res in rx.iter() {
        let (d, path) = res.unwrap();
        println!("path: {:?}, digest: {:?}", path, d);
        debug!("path: {:?}, digest: {:?}", path, d);
    }
    
    std::thread::scope(|t|{
        t.spawn(||{
            std::thread::sleep(std::time::Duration::from_millis(400));
            println!("这里是子线程");
            std::thread::sleep(std::time::Duration::from_millis(400));
        });
    });

    println!("这里是主线程");
}
