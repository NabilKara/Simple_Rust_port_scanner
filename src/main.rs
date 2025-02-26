use clap::Parser;
use std::net::IpAddr;
use tokio::net::TcpStream;
use colored::*;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;
use cidr::*;

#[derive(Debug,Parser)]
struct  Args{
    #[arg(conflicts_with("cidr"), required_unless_present("cidr"))]
    addr: Option<IpAddr>,

    #[arg(long)]
    cidr: Option<IpCidr>,

    #[arg(long, default_value_t = 1)]
    port_start: u16,

    #[arg(long, default_value_t = 1024)]
    port_end: u16,
}
async fn scan(addr: IpAddr, port: u16, results_tx : Sender<(IpAddr, u16)>) -> Result<(), SendError<(IpAddr, u16)>> {
     let connection_attempt = TcpStream::connect((addr, port)).await;
    match connection_attempt {
        Ok(_) => {
            results_tx.send((addr, port)).await?;
        }
        Err(_) => {}
    }
    Ok(())
}
#[tokio::main]
async fn main(){
    let args = Args::parse();
    assert!(args.port_end >= args.port_start);


    let mut tasks_per_network = (args.port_end - args.port_start) as usize;
    let(tx, mut rx) = mpsc::channel(100);
    let mut tasks = Vec::with_capacity(tasks_per_network);
    let (mut from_single, mut from_cidr);
    let addrs : &mut dyn Iterator<Item = IpAddr> =
    if let Some(addr) = args.addr {
        from_single = vec![addr].into_iter();
        &mut from_single
    }else if let Some(network) = args.cidr {
        from_cidr = network.iter().map(|ip_inet| ip_inet.first_address());
        &mut from_cidr
    }else{
        unreachable!()
    };

    for addr in addrs {
        println!("? {addr}:{}-{}", args.port_start, args.port_end);
        for port in args.port_start..args.port_end {
            let tx = tx.clone();
            let task = tokio::spawn(async move {
                if let Err(err) = scan(addr, port, tx).await {
                    eprintln!("error: {err}")
                }
            });
            tasks.push(task);
        }
    }
    for task in tasks {
            let _ = task.await;
        }
    drop(tx);

        println!("\n{}", "Open Ports Found:".green());
        while let Some((addr, port)) = rx.recv().await {
            println!("{}:{} is OPEN", addr, port);
        }
        println!("{}", "Scan Complete !".green());
}
