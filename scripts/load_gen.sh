#!/bin/bash
sudo sysctl -w net.inet.tcp.msl=1000
ulimit -n 200000 && siege -f siege.txt -r 1000000 -c 255 -b
