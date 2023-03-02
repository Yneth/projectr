#!/bin/bash
curl -X POST "localhost:9200/test_index/_delete_by_query" -H 'Content-Type: application/json' -d' { "query": { "match_all":{} } } '
