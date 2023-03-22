fn excute_op() -> Result<(), &'static str>{
    Err("this is err msg")
}

pub fn test(){
    env_logger::init();
    log::info!("123123");
    
    if let Err(str) = excute_op() {
        log::error!("error occured {}", str);
    }

}