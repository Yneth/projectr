# Description

This project is a sample to monitor application under load using TIG monitoring stack. TIG is an acronym for telegraf, influxdb and grafana.

* Telegraf is an agent for log collection and distribution.
* InfluxDB is a storage for collected metrics.
* Grafana is a dash-boarding system.

Application includes the following services:
* loadbalancer - in my case it is nginx;
* mongodb
* elasticsearch
* API - service with two endpoints:
  1) /insert - inserts random data to mongo and elastic;
  2) /read - reads data from both storages;
  
`Rust` was used for API service. It is a systems language that made a lot of progress in web simplifying development and making it more user-friendly for new developers saving the benefits of the systems' language. Applications written in Rust consume way less memory and CPU comparing to Java/Python/Golang as instead of runtime GC it uses compile time memory management implemented via borrow checker.

`siege` CLI tool was used for load testing. 

System configurations for siege tool:

* `sysctl -w net.inet.tcp.msl=1000` 
   on MacOS there is a limit in maximum number of ports equal to 16k.
   this property manages the time before socket is released back to the system TCP stack;
* `ulimit -n 200000`
  sets hard and soft limit for maximum number of opened file descriptors. In system each socket has its own file descriptor. Number of those descriptors is limited by the system. Using this command you can overcome limitations giving your application to open more sockets at the moment of time.

# Results

Results are formed by `siege` command running 300K requests to the API endpoints with 255 concurrency.

## Network (System)
![network.png](images%2Fnetwork.png)

## Network (bmon)
![bmon.png](images%2Fbmon.png)

## CPU and memory (System)
![cpu_mem.png](images%2Fcpu_mem.png)

## Disk IO (System)
![disk.png](images%2Fdisk.png)

## Nginx
![nginx.png](images%2Fnginx.png)

## Elastic
![elastic_cpu.png](images%2Felastic_cpu.png)
![elastic_fd.png](images%2Felastic_fd.png)
![elastic_gc.png](images%2Felastic_gc.png)

## Mongo
![mongo.png](images%2Fmongo.png)

## docker services CPU
![docker_all_cpu.png](images%2Fdocker_all_cpu.png)

## docker services Network
![docker_all_net.png](images%2Fdocker_all_net.png)

## siege report

```
Lifting the server siege...
Transactions:                 277752 hits
Availability:                  99.95 %
Elapsed time:                1062.20 secs
Data transferred:             335.39 MB
Response time:                  0.97 secs
Transaction rate:             261.49 trans/sec
Throughput:                     0.32 MB/sec
Concurrency:                  254.63
Successful transactions:      277752
Failed transactions:             131
Longest transaction:            6.40
Shortest transaction:           0.02
```

## Analysis
System did not fail under load.

For some reason telegraf incorrectly tracks packet rate. It can be validated using bmon screenshot from API container.

The most CPU consuming application was `elasticsearch` which could quickly become a bottleneck if not scaled properly. It can be validated using docker CPU screen. It is due to the nature of JVM that requires GC at runtime. On the later stages of the siege it can be seen that it takes significantly more time for garbage collection.

It is important to mention that my elasticsearch query may be optimized by caching as I forgot to add sorting, and it always gives the first ten results.

Also, it would be nice to test using separate endpoints for mongo/elastic read/write with customisable load configuration to see system behaviour with different scenarios. For example having 25% to read and 75% to write.