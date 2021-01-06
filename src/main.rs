use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::time::Duration;

struct Arguments {
	flag: String,
	ip_address: IpAddr,
	threads: u16,
}

impl Arguments {
	fn new(args: &[String]) -> Result<Arguments, &'static str> {
		if args.len() < 2 {
			return Err("Too Few Arguments");
		}
		if args.len() > 4 {
			return Err("Too Many Arguments");
		}

		let address_string = args[1].clone();

		if let Ok(ip_address) = IpAddr::from_str(&address_string) {
			return Ok(Arguments { flag: String::from(""), ip_address, threads: 4 });
		} else {
			let flag = args[1].clone();

			if (flag.contains("-h") || flag.contains("--help")) && args.len() == 2 {
				println!("Options: \r\n-t --threads \t\t choose the number of threads");
				println!("-h --help \t\t view the message");
				return Err("help");
			} else if (flag.contains("-h") || flag.contains("--help"))
				&& (flag.contains("-t") || flag.contains("--threads")) {
				return Err("Flag Error: Too Many Arguments");
			} else if flag.contains("-t") || flag.contains("--threads") {
				let ip_address = match IpAddr::from_str(&args[3]) {
					Ok(ip) => ip,
					Err(_) => return Err("IP Address Error: must follow IpV4 or IpV6 format"),
				};
				let threads = match args[2].parse::<u16>() {
					Ok(thread) => thread,
					Err(_) => return Err("Thread Error: invalid number"),
				};

				return Ok(Arguments { flag, ip_address, threads });
			} else {
				return Err("Argument Error: invalid syntax");
			}
		}
	}
}

fn scan(transmitter: Sender<u16>, start_port: u16, address: IpAddr, threads: u16) {
	const MAX_PORT_NUMBER: u16 = 65535;
	let mut port: u16 = start_port + 1;
	let timeout = Duration::new(5, 0);
	let socket_address = SocketAddr::new(address, port);

	loop {
		match TcpStream::connect_timeout(&socket_address, timeout) {
			Ok(_) => {
				print!("Checking Port #{0}", port);
				io::stdout().flush().unwrap();
				transmitter.send(port).unwrap();
			},
			Err(_) => {
				println!("Port #{0} closed", port);
			}
		}

		if (MAX_PORT_NUMBER - port) <= threads {
			break;
		}
		port += threads;
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();
	let arguments = Arguments::new(&args).unwrap_or_else(
		|error| {
			if error.contains("help") {
				process::exit(0);
			} else {
				eprintln!("{0}: {1}", program, error);
				process::exit(0);
			}
		}
	);

	let thread_count = arguments.threads;
	let address = arguments.ip_address;
	let (transmitter, receiver) = channel();
	for thread in 0..thread_count {
		let single_transmitter = transmitter.clone();

		thread::spawn(move || {
			scan(single_transmitter, thread, address, thread_count);
		});
	}

	let mut port_list = vec![];
	drop(transmitter);
	for output in receiver {
		port_list.push(output);
	}

	println!();
	port_list.sort();
	for number in port_list {
		println!("Port #{0} is open", number);
	}
}
