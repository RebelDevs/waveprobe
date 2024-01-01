//use std::{fs,io};

pub fn run(_script: String, _args: String) -> String {
    return concat!(
        "PING google.com (2a00:1450:4019:808::200e) 56 data bytes\n",
        "64 bytes from fjr04s12-in-x0e.1e100.net (2a00:1450:4019:808::200e): icmp_seq=1 ttl=59 time=6.71 ms\n",
        "64 bytes from fjr04s12-in-x0e.1e100.net (2a00:1450:4019:808::200e): icmp_seq=2 ttl=59 time=8.10 ms\n",
        "--- google.com ping statistics ---\n",
        "4 packets transmitted, 4 received, 0% packet loss, time 3006ms\n",
        "rtt min/avg/max/mdev = 6.710/8.459/9.940/1.202 ms\n",
    ).to_string();
}
