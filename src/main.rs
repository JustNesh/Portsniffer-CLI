use std::{env, process};
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{Sender ,channel};
use std::thread;

const MAX: u16 = 65535;

struct Arguments{
    flag: String,
    ipaddress: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        let args_length = args.len();
        if args_length < 2 {
            return Err("Not enough arguments")
        } else if args_length > 4 {
            return Err("Too many arguments")
        } 
        let first_arg = args[1].clone();
        // This if let checks to see if the first argument is able to be turned into an ipaddress.
        if let Ok(ipaddr) = IpAddr::from_str(&first_arg){
            return Ok(Arguments{flag: String::from(""), ipaddress: ipaddr, threads: 4})
        } else // If an error is returned, that probably means a flag was given or  
            {
            let flag = first_arg;
            if (flag.contains("-h") || flag.contains("-help")) && args_length == 2{
                println!("Usage: -j to select how many threads you want
                \r\n      -h or -help to show this message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments")
            } else if flag.contains("-j") {
                let ipaddress = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IPADDR; must be IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number")
                };
                return Ok(Arguments { flag, ipaddress, threads })
            } else {
                return Err("Invalid syntax")
            }
        }

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let args = Arguments::new(&args.as_slice()).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            println!("{} problem parsing arguments: {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = args.threads;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, args.ipaddress, num_threads);
        });
    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr,port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) < num_threads {
            break;
        }
        port += num_threads;
    }
}   