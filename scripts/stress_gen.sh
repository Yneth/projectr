#!/bin/bash
sudo sysctl -w net.inet.tcp.msl=1000
echo "=================== 10 users ==================="
echo "$(date)"
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=10 --benchmark --json-output --quiet --log
echo "=================== 50 users ==================="
echo "$(date)"
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=50 --benchmark --json-output --quiet --log
echo "=================== 100 users ==================="
echo "$(date)"
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=100 --benchmark --json-output --quiet --log
echo "=================== 200 users ==================="
echo "$(date)"
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=200 --benchmark --json-output --quiet --log
echo "=================== 500 users ==================="
echo "$(date)"
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=500 --benchmark --json-output --quiet --log
echo "=================== 1000 users =================="
echo "$(date)"
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=1000 --benchmark --json-output --quiet --log
