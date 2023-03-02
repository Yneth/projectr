# Description

Stress testing of the sample Rust application.
In scope of this test we want to see how application 
handles the load under stress using `siege` tool with different configurations of concurrent users.

# Setup

* Elasticsearch with 100K records in data index;
* MongoDB with 100K records in data collection;
* Rust web app using axum web framework with single `/index` endpoint;
* `siege` bash tool;

## Elasticsearch data index
```json
{
  "test_index": {
    "mappings": {
      "properties": {
        "data": {
          "type": "text",
          "fields": {
            "keyword": {
              "type": "keyword",
              "ignore_above": 256
            }
          }
        },
        "time": {
          "type": "long"
        }
      }
    }
  }
}
```

## MongoDB collections

### data collection
```bash
test_db> db.test_coll.find().next()
{
  _id: '64a66dd2-c0b5-4413-8f40-ceedacb3d742',
  data: 'Djx1wXF7mCuzFovC'
}
```

### counter collection
```bash
test_db> db.counter.find()
[ { _id: 'view_counter', value: 1915644 } ]
```


## API: GET /index

When requested, will increase hit mongodb counter and return an HTML including latest 10 records from data index of elasticsearch and 10 latest records from data collection of mongodb.

### Request

Endpoint does not accept any kind of request parameters or request body.

### Response body

Returns html.

### curl example

```bash
http :5000/index
HTTP/1.1 200 OK
content-length: 3093
content-type: text/html; charset=utf-8
date: Thu, 02 Mar 2023 19:31:42 GMT

<html><head><title>index</title><head><body><pre>view count: 1915643
time: 1677785502534
mongo: [{"_id":"a3f1c249-8bcc-48a7-844a-3e98b0603581","data":"Jq7ENZtnmLhQqNhv"},{"_id":"fcc30c4e-4d12-42b1-9d6e-d3a006a2eb7c","data":"ceIHqVhpDsiLJpyh"},{"_id":"2413597b-8ad2-4623-9ede-7ac1ae41ab84","data":"4xvFIJQPv5KgNoCu"},{"_id":"a300d49a-98ec-450c-86cb-9fb769e8a148","data":"xA43ShsYjegKKeho"},{"_id":"f4e832b7-eda3-47c3-bc27-f6b93f13c280","data":"coaMHAgVj6WsvLQh"},{"_id":"db59b7a2-aeff-4be7-9491-53eb6af886a6","data":"NpSiVtxwn3AsOdXK"},{"_id":"14e194bb-532a-4928-958a-d645cdffd1f1","data":"N1YXTu9j7mqyZ9J5"},{"_id":"b0028a8a-23d0-49a8-807b-33c18b73fcda","data":"fu5g9XY5Ef0ZioVV"},{"_id":"837fe72b-0f4c-461f-bf7a-00b745269bc0","data":"cuHpdnuJWJ1Aew0N"},{"_id":"0f2b7aaa-fafb-4d4c-bc1b-5b0c974868c5","data":"EBX6Ba3osX8GDWU2"}]
elastic: "{\"took\":22,\"timed_out\":false,\"_shards\":{\"total\":1,\"successful\":1,\"skipped\":0,\"failed\":0},\"hits\":{\"total\":{\"value\":10000,\"relation\":\"gte\"},\"max_score\":null,\"hits\":[{\"_index\":\"test_index\",\"_type\":\"_doc\",\"_id\":\"22eacd44-1f20-4a46-98af-f32826aed2ba\",\"_score\":null,\"_source\":{\"data\":\"S5oIf5NytTCEShNh\",\"time\":1677744820519},\"sort\":[1677744820519]},{\"_index\":\"test_index\",\"_type\":\"_doc\",\"_id\":\"41ff8c3c-15d7-425e-bcde-c2747ea4aff3\",\"_score\":null,\"_source\":{\"data\":\"fnE6Uz9rQ47IvU9n\",\"time\":1677744819918},\"sort\":[1677744819918]},{\"_index\":\"test_index\",\"_type\":\"_doc\",\"_id\":\"9cfe6bd3-4314-44f0-a7cd-8d187e9bb06f\",\"_score\":null,\"_source\":{\"data\":\"ppdQYMwx5sj4GYF0\",\"time\":1677744819881},\"sort\":[1677744819881]},{\"_index\":\"test_index\",\"_type\":\"_doc\",\"_id\":\"07374b1a-46ea-4faa-b104-bf6e59834ad0\",\"_score\":null,\"_source\":{\"data\":\"plCodRoAW0WkNdWE\",\"time\":1677744819854},\"sort\":[1677744819854]},{\"_index\":\"test_index\",\"_type\":\"_doc\",\"_id\":\"01f15713-df1b-4f4f-919b-94271f298977\",\"_score\":null,\"_source\":{\"data\":\"JE6zDqHnXoQ2jSMc\",\"time\":1677744819833},\"sort\":[1677744819833]},{\"_index\":\"test_index\",\"_type\":\"_doc\",\"_id\":\"82b8c031-b43d-4237-9fb7-1df4057d05ba\",\"_score\":null,\"_source\":{\"data\":\"y64OCk2uN83bK586\",\"time\":1677744819746},\"sort\":[1677744819746]},{\"_index\":\"test_index\",\"_type\":\"_doc\",\"_id\":\"13cf53f1-e54b-4d32-ac5c-30b8808af1b0\",\"_score\":null,\"_source\":{\"data\":\"NN5vwbysos2XFFIk\",\"time\":1677744819745},\"sort\":[1677744819745]},{\"_index\":\"test_index\",\"_type\":\"_doc\",\"_id\":\"4eb1d0b2-5082-4f3b-b709-b4c5b8435fdf\",\"_score\":null,\"_source\":{\"data\":\"y0cLcDWgvTjbAu4x\",\"time\":1677744819716},\"sort\":[1677744819716]},{\"_index\":\"test_index\",\"_type\":\"_doc\",\"_id\":\"1ed5b4c9-1cca-4827-924e-87ead7bcdb72\",\"_score\":null,\"_source\":{\"data\":\"8O72YSDJkJAmW6AB\",\"time\":1677744819639},\"sort\":[1677744819639]},{\"_index\":\"test_index\",\"_type\":\"_doc\",\"_id\":\"510e68c0-1f80-428f-8b5e-f82590063662\",\"_score\":null,\"_source\":{\"data\":\"5d8xNuDdwUJ5d8pD\",\"time\":1677744819594},\"sort\":[1677744819594]}]}}"
</pre></body></html>
```

## siege configuration

```bash
#!/bin/bash
echo "=================== 10 users ==================="
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=10 --benchmark --json-output --quiet --log

echo "=================== 50 users ==================="
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=50 --benchmark --json-output --quiet --log

echo "=================== 100 users ==================="
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=100 --benchmark --json-output --quiet --log

echo "=================== 200 users ==================="
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=200 --benchmark --json-output --quiet --log

echo "=================== 500 users ==================="
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=500 --benchmark --json-output --quiet --log

echo "=================== 1000 users =================="
ulimit -n 200000 && siege -q --file=stress.txt --reps=1000 --concurrent=1000 --benchmark --json-output --quiet --log
```

Where `stress.txt` has only one record: `/index`.

# Result

| property\concurrency      | 10       | 50       | 100      | 200      | 500      | 1000     |
  |---------------------------|----------|----------|----------|----------|----------|----------|
|  transactions             | 10000    | 50000    | 100000   | 199980   | 143196   | 33995    |
|  availability             | 100      | 100      | 100      |  99.99   |  99.19   |  96.58   |
|  elapsed_time             |  60.82   |  323.91  |  298.67  |  645.93  |  357.75  |  93.17   |
|  data_transferred         |  29.5    |  147.51  |  295.06  |  590.06  |  422.56  |  100.34  |
|  response_time            |  0.06    |  0.32    |  0.3     |  0.64    |  1.24    |  2.69    |
|  transaction_rate         |  164.42  |  154.36  |  334.82  |  309.6   |  400.27  |  364.87  |
|  throughput               |  0.48    |  0.46    |  0.99    |  0.91    |  1.18    |  1.08    |
|  concurrency              |  9.84    |  49.8    |  99.73   |  198.88  |  497.54  |  983.25  |
|  successful_transactions  | 10000    | 50000    | 100000   | 199980   | 143196   | 33995    |
|  failed_transactions      | 0        | 0        | 0        | 20       | 1165     | 1203     |
|  longest_transaction      |  1.12    |  31.44   |  1.33    |  2.58    | 5        |  7.46    |
|  shortest_transaction     |  0.01    |  0.01    |  0.01    |  0.01    |  0.03    |  0.05    |


# Analysis

Application successfully handled up to 100 CU(concurrent users), after that hit mark it started degrading significantly, mostly due to connection failures.

## CPU
![stress_cpu.png](images%2Fstress_cpu.png)

## Memory / Network / Disk

![stress_mem_nw_disk.png](images%2Fstress_mem_nw_disk.png)
