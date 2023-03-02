#!/bin/bash
mongosh -u admin -p admin \
  --eval 'use db_test' \
  --eval 'db.test_coll.deleteMany({})'
