use std::env;
use std::net::IpAddr;
use std::str::FromStr;

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
				println!("Options: \r\n -t --threads \t\t choose the number of threads");
				println!("-h --help \t\t view the message");
				return Err("help");
			} else if (flag.contains("-h") || flag.contains("--help"))
				|| (flag.contains("-t") || flag.contains("--threads")) {
				return Err("Flag Error: Too Many Arguments");
			} else if (flag.contains("-t") || flag.contains("--threads")) && args.len() == 4 {
				let ip_address = match IpAddr::from_str(&args[3]) {
					Ok(ip) => ip,
					Err(_) => println!("IP Address Error: must follow IpV4 or IpV6 format"),
				};
				let threads = match args[2].parse::<u16>() {
					Ok(thread) => thread,
					Err(_) => println!("Thread Error: invalid number"),
				};

				return Ok(Arguments { flag, ip_address, threads });
			} else {
				return Err("Argument Error: Invalid syntax");
			}
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = &args[0];
}
