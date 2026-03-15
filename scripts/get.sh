#!/bin/sh
curl -X GET http://0.0.0.0:3000/users/user/"$2" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $1" \
