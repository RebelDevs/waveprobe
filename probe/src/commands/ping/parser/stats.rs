use regex::Regex;

#[derive(Debug, Default)]
pub struct RTT {
    min: f32,
    avg: f32,
    max: f32,
    mdev: f32,
}

#[derive(Debug, Default)]
pub struct Packets {
    total: i8,
    loss: i8,
    rcv: i8,
    drop: i8,
}

pub fn extract(data: &str) -> (RTT, Packets) {
    return (extract_rtt(data), extract_packets(data));
}

fn extract_rtt(data: &str) -> RTT {
    let mut rtt = RTT::default();
    let re =
        Regex::new(r"rtt min/avg/max/mdev = ([\d\.]+)/([\d\.]+)/([\d\.]+)/([\d\.]+) ms").unwrap();

    re.captures(data).map(|x| {
        match x[1].parse::<f32>() {
            Ok(num) => rtt.min = num,
            Err(e) => println!("Failed to convert rtt min, {}", e),
        }
        match x[2].parse::<f32>() {
            Ok(num) => rtt.avg = num,
            Err(e) => println!("Failed to convert rtt avg, {}", e),
        }
        match x[3].parse::<f32>() {
            Ok(num) => rtt.max = num,
            Err(e) => println!("Failed to convert rtt max, {}", e),
        }
        match x[4].parse::<f32>() {
            Ok(num) => rtt.mdev = num,
            Err(e) => println!("Failed to convert rtt mdev, {}", e),
        }
    });

    return rtt;
}

fn extract_packets(data: &str) -> Packets {
    let mut packets = Packets::default();
    let re = Regex::new(r"(\d+) packets transmitted, (\d+) received, (\d+)% packet loss").unwrap();
    re.captures(data).map(|x| {
        match x[1].parse::<i8>() {
            Ok(num) => packets.total = num,
            Err(e) => println!("Failed to convert packets total, {}", e),
        }
        match x[2].parse::<i8>() {
            Ok(num) => packets.rcv = num,
            Err(e) => println!("Failed to convert packets received, {}", e),
        }
        match x[3].parse::<i8>() {
            Ok(num) => packets.loss = num,
            Err(e) => println!("Failed to convert packets loss, {}", e),
        }
    });

    packets.drop = packets.total - packets.rcv;

    return packets;
}
