use etcd_client::{Client, ConnectOptions, Error};
use clap::{Command, Arg};

pub mod backtrace;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Command::new("etcd-version-history")
        .author("zzy")
        .version("1.0.0")
        .about("etcd version history tools")
        .arg(
            Arg::new("addrs")
                .long("addrs")
                .help("etcd cluster addrs e.g. 10.1.0.10:2379,10.1.0.11:2379,10.1.0.12:2379")
                .required(true)
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .help("etcd user")
                .required(false)
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .help("etcd password")
                .required(false)
        )
        .arg(
            Arg::new("keys")
                .long("keys")
                .help("List of keys to be queried. e.g. foo,bar")
                .required(true)
        )
        .get_matches();

    let addrs = args.get_one::<String>("addrs").map(String::as_str).unwrap();
    let addrs: Vec<&str> = addrs.split(",").collect();
    if addrs.len() == 0 {
        println!("etcd addrs error: {:?}", addrs);
        return Err(Error::InvalidArgs(String::from("addrs")));
    }
    // println!("{:?}", addr_arr);

    let keys = args.get_one::<String>("keys").map(String::as_str).unwrap();
    let keys: Vec<&str> = keys.split(",").collect();
    if keys.len() == 0 {
        println!("input keys error: {:?}", keys);
        return Err(Error::InvalidArgs(String::from("keys")));
    }
    println!("input keys: {:?}", keys);

    let user = args.get_one::<String>("user").map(String::as_str).unwrap_or("");
    let password = args.get_one::<String>("password").map(String::as_str).unwrap_or("");

    let mut client_options = None;
    if user != "" {
        client_options = Some(ConnectOptions::new()
            .with_user(user, password));
    }

    let mut client = Client::connect(&addrs, client_options).await?;

    for key in keys {
        println!("=========================== tracking key: {} ===========================", key);
        let res = backtrace::tracking(&mut client, key).await;
        match res {
            Ok(hvs) => {
                if hvs.len() > 0 {
                    let hv = hvs.get(0).unwrap();
                    println!("create version: {} number of versions: {}", 
                        hv.get_create_version(), hvs.len());
                }
                
                for hv in hvs {
                    println!("{}", hv);
                }
            },
            Err(err) => {
                println!("error: {}", err);
            }
        }
    }

    Ok(())
}
