use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::io::{ErrorKind, Write};
use std::{cmp, fs::File, io::Read};
use sysinfo::{Networks, System};
#[derive(Deserialize, Serialize)]
struct Config {
    img: String,
    memory: String,
    cpu: String,
    sysname: String,
    kernelver: String,
    osver: String,
    hostname: String,
    network: String,
}

fn main() {
    let dir = BaseDirs::new().unwrap();

    let config_file = File::open(dir.config_local_dir().join("nfetch.toml").to_str().unwrap());
    let mut config_fs = match config_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => create_base_config(),
            _ => panic!("Some problem! {error:?}"),
        },
    };

    let mut cfg_str = String::new();
    config_fs.read_to_string(&mut cfg_str).unwrap();
    let config: Config = toml::from_str(&cfg_str).unwrap();
    let mut sys = System::new_all();
    let mut data: Vec<String> = Vec::new();

    let tst = &config.img;

    let width = get_max_line_len(&tst) + 5;

    sys.refresh_all();

    load_data(&mut data, &config, &sys);

    let mut lines = tst.lines();

    let lnum = lines.clone().count();

    let len = cmp::max(data.len(), lnum);

    for i in 0..len {
        if (i < lnum) & (i < data.len()) {
            let line = format!("{:>5} {:<width$}", " ", lines.next().unwrap());
            println!("{}  {}", line, data[i]);
        } else if i < data.len() {
            println!("{:>width$}  {}", " ", data[i]);
        } else if (i >= data.len()) & (i < lnum) {
            let line = format!("{:>5} {:<width$}", " ", lines.next().unwrap());
            println!("{} ", line);
        }
    }
}

fn load_data(data: &mut Vec<String>, config: &Config, sys: &System) {
    if !config.hostname.is_empty() {
        data.push(format!(
            "{} : {:?}",
            config.hostname,
            System::host_name().unwrap()
        ));
    }

    if !config.sysname.is_empty() {
        data.push(format!(
            "{} : {:?}",
            config.sysname,
            System::name().unwrap()
        ));
    }

    if !config.osver.is_empty() {
        data.push(format!(
            "{} : {:?}",
            config.osver,
            System::os_version().unwrap()
        ));
    }

    if !config.kernelver.is_empty() {
        data.push(format!(
            "{} : {:?}",
            config.kernelver,
            System::kernel_version().unwrap()
        ));
    }

    if !config.cpu.is_empty() {
        data.push(format!("{} : {} ", config.cpu, sys.cpus()[0].brand()));
    }

    if !config.memory.is_empty() {
        data.push(format!(
            "{} : {} MB",
            config.memory,
            sys.total_memory() / 1000000
        ));
    }
    if !config.network.is_empty() {
        data.push(format!("{} : {} ", config.network, ""));
        let net = Networks::new_with_refreshed_list();
        let mut _i = 0;
        for (interface, netd) in &net {
            data.push(format!(
                "  {} : {:?} ",
                interface,
                netd.ip_networks()[0].addr.to_string()
            ));
            _i += 1;
        }
    }
}

fn get_max_line_len(text: &String) -> usize {
    let mut res: usize = 0;
    for line in text.lines() {
        if line.len() > res {
            res = line.chars().count();
        }
    }
    res
}

fn create_base_config() -> File {
    let base_config: Config = Config {
        img: "
⠄⠄⠄⠄⢠⣿⣿⣿⣿⣿⢻⣿⣿⣿⣿⣿⣿⣿⣿⣯⢻⣿⣿⣿⣿⣆⠄⠄⠄
⠄⠄⣼⢀⣿⣿⣿⣿⣏⡏⠄⠹⣿⣿⣿⣿⣿⣿⣿⣿⣧⢻⣿⣿⣿⣿⡆⠄⠄
⠄⠄⡟⣼⣿⣿⣿⣿⣿⠄⠄⠄⠈⠻⣿⣿⣿⣿⣿⣿⣿⣇⢻⣿⣿⣿⣿⠄⠄
⠄⢰⠃⣿⣿⠿⣿⣿⣿⠄⠄⠄⠄⠄⠄⠙⠿⣿⣿⣿⣿⣿⠄⢿⣿⣿⣿⡄⠄
⠄⢸⢠⣿⣿⣧⡙⣿⣿⡆⠄⠄⠄⠄⠄⠄⠄⠈⠛⢿⣿⣿⡇⠸⣿⡿⣸⡇⠄
⠄⠈⡆⣿⣿⣿⣿⣦⡙⠳⠄⠄⠄⠄⠄⠄⢀⣠⣤⣀⣈⠙⠃⠄⠿⢇⣿⡇⠄
⠄⠄⡇⢿⣿⣿⣿⣿⡇⠄⠄⠄⠄⠄⣠⣶⣿⣿⣿⣿⣿⣿⣷⣆⡀⣼⣿⡇⠄
⠄⠄⢹⡘⣿⣿⣿⢿⣷⡀⠄⢀⣴⣾⣟⠉⠉⠉⠉⣽⣿⣿⣿⣿⠇⢹⣿⠃⠄
⠄⠄⠄⢷⡘⢿⣿⣎⢻⣷⠰⣿⣿⣿⣿⣦⣀⣀⣴⣿⣿⣿⠟⢫⡾⢸⡟⠄.
⠄⠄⠄⠄⠻⣦⡙⠿⣧⠙⢷⠙⠻⠿⢿⡿⠿⠿⠛⠋⠉⠄⠂⠘⠁⠞⠄⠄⠄
⠄⠄⠄⠄⠄⠈⠙⠑⣠⣤⣴⡖⠄⠿⣋⣉⣉⡁⠄⢾⣦⠄⠄⠄⠄⠄⠄⠄⠄
        "
        .to_string(),
        memory: "RAM".to_string(),
        cpu: "CPU".to_string(),
        sysname: "System".to_string(),
        kernelver: "Kernel Version".to_string(),
        osver: "OS Version".to_string(),
        hostname: "Host Name".to_string(),
        network: "Network".to_string(),
    };
    let toml = toml::to_string(&base_config).unwrap();
    let dir = BaseDirs::new().unwrap();

    let mut file = File::create(dir.config_local_dir().join("nfetch.toml")).unwrap();
    file.write_all(toml.as_bytes()).unwrap();

    File::open(dir.config_local_dir().join("nfetch.toml")).unwrap()
}
