#!/bin/bash

# Initialize ARGS as an empty array
ARGS=()

# Populate ARGS with all arguments starting from the second, if any
if [ $# -gt 0 ]; then
    ARGS=( "${@:1}" )
fi

# Handling the case when ARGS is empty
if [ ${#ARGS[@]} -eq 0 ]; then
    TARGET=""
else
    # Get the last element of the ARGS array
    TARGET=${ARGS[${#ARGS[@]}-1]}
fi

if [ "$TARGET" = "error" ]; then
  >&2 echo "error"
  exit 1;
fi

echo "PING success (1.2.3.4) 56(84) bytes of data.
64 bytes from success (1.2.3.4): icmp_seq=1 ttl=60 time=3.34 ms
64 bytes from success (1.2.3.4): icmp_seq=2 ttl=60 time=3.35 ms
64 bytes from success (1.2.3.4): icmp_seq=3 ttl=60 time=3.38 ms

--- google.com ping statistics ---
3 packets transmitted, 3 received, 0% packet loss, time 401ms
rtt min/avg/max/mdev = 3.339/3.358/3.384/0.019 ms"
exit 0;
