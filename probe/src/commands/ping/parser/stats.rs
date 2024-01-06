use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RTT {
    pub min: f32,
    pub avg: f32,
    pub max: f32,
    pub mdev: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Packets {
    pub total: i8,
    pub loss: i8,
    pub rcv: i8,
    pub drop: i8,
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

#[cfg(test)]
mod tests_rtt {
    use super::*;

    #[test]
    fn success() {
        let data = "rtt min/avg/max/mdev = 4.779/4.802/4.817/0.016 ms";
        let result = extract_rtt(data);

        assert_eq!(result.min, 4.779);
        assert_eq!(result.avg, 4.802);
        assert_eq!(result.max, 4.817);
        assert_eq!(result.mdev, 0.016);
    }

    #[test]
    fn no_match() {
        let data = "rtt min/avg/max/mdev = nodata ms";
        let result = extract_rtt(data);

        assert_eq!(result.min, 0.0);
        assert_eq!(result.avg, 0.0);
        assert_eq!(result.max, 0.0);
        assert_eq!(result.mdev, 0.0);
    }
}

#[cfg(test)]
mod tests_packets {
    use super::*;

    #[test]
    fn success() {
        let data = "3 packets transmitted, 3 received, 0% packet loss, time 402ms";
        let result = extract_packets(data);

        assert_eq!(result.total, 3);
        assert_eq!(result.rcv, 3);
        assert_eq!(result.loss, 0);
        assert_eq!(result.drop, 0);
    }

    #[test]
    fn drop() {
        let data = "3 packets transmitted, 0 received, 0% packet loss, time 402ms";
        let result = extract_packets(data);

        assert_eq!(result.total, 3);
        assert_eq!(result.rcv, 0);
        assert_eq!(result.loss, 0);
        assert_eq!(result.drop, 3);
    }

    #[test]
    fn no_match() {
        let data = " packets transmitted,malformed received, 0% packet loss, time 402ms";
        let result = extract_packets(data);

        assert_eq!(result.total, 0);
        assert_eq!(result.rcv, 0);
        assert_eq!(result.loss, 0);
        assert_eq!(result.drop, 0);
    }
}
