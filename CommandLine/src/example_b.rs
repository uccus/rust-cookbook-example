use ansi_term::{Colour,Style};

pub fn test(){
    println!("this is {}, {}, {}, {}"
                , Colour::Red.paint("red")
                , Colour::Green.paint("green")
                , Colour::Cyan.bold().paint("cyan")
                , Colour::Yellow.paint("yellow"));
    println!("这是{}"
                , Style::new().bold().paint("粗体字"));
}