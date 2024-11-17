#!/bin/bash

# Find the PID of the process listening on port 4221
pid=$(netstat -tulpn | grep LISTEN | grep "127.0.0.1:4221" | awk '{print $7}' | cut -d'/' -f1)

if [ -z "$pid" ]; then
  echo "No process found listening on 127.0.0.1:4221"
  exit 1
fi

echo "Found PID: $pid"

# Find the file descriptor of the process listening on port 4221
fd=$(lsof -np $pid | grep "TCP 127.0.0.1:4221 (LISTEN)" | awk '{print $4}' | sed 's/u$//')

if [ -z "$fd" ]; then
  echo "No file descriptor found for PID $pid listening on 127.0.0.1:4221"
  exit 1
fi

echo "Found file descriptor: $fd"

# Use gdb to close the file descriptor
gdb -p $pid <<EOF
call close($fd)
detach
quit
EOF

echo "Closed file descriptor $fd for PID $pid"
