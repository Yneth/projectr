#!/bin/bash

elasticdump --input=http://localhost:9200/test_index --output=./data/test_index_mapping.json --type=mapping

elasticdump --input=http://localhost:9200/test_index --output=./data/test_index_mapping.json --type=data

mongodump --uri="mongodb://admin:admin@localhost:27017"

