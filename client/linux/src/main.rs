// Luminum Client for Linux
// by Christopher R. Curzio <ccurzio@luminum.net>

use clap::{Arg, App};
use colored::Colorize;
use chrono::Local;
use chrono::format::strftime::StrftimeItems;
use std::process;
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::fs::{self, File};

const VER: &str = "0.0.1";
const DCFGPATH: &str = "/opt/luminum/LuminumClient/conf/server.conf.db";

fn main() {
	let matches = App::new("Luminum Server Daemon")
		.version(VER)
		.author("Christopher R. Curzio <ccurzio@luminum.net>")
	.arg(Arg::with_name("config")
		.short('c')
		.long("config")
		.value_name("CFG_PATH")
		.help("Specifies the path to the configuration database")
		.takes_value(true))
	 .arg(Arg::with_name("debug")
		.short('d')
		.long("debug")
		.value_name("debug")
		.help("Enables debug mode")
		.takes_value(false))
	.get_matches();

	let config_file = matches.value_of("config").unwrap_or(DCFGPATH);
	let debug = matches.is_present("debug");

	let mut clientconfig: HashMap<String, String> = HashMap::new();

	if fs::metadata(config_file).is_ok() {
		// Read Client Config
		}
	else {
		dbout(debug,1,format!("Configuration database not found.").as_str());
		process::exit(1);
		}
	}

fn dbout(debug: bool, outlvl: i32, output: &str) {
	let dateformat = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
	let current_datetime = Local::now();
	let formatted_datetime = current_datetime.format_with_items(dateformat).to_string();
	let mut etype = String::new();

	if debug {
		if outlvl == 0 { etype = "PROC".cyan().to_string(); }
		else if outlvl == 1 { etype = "FAIL".red().to_string(); }
		else if outlvl == 2 { etype = "WARN".yellow().to_string(); }
		else if outlvl == 3 { etype = " OK ".green().to_string(); }
		else if outlvl == 4 { etype = "INFO".to_string(); }
		println!("{} [{}] {}",formatted_datetime,etype,output);
		}
	else {
		if outlvl == 1 { println!("{}",output); }
		}
	}
