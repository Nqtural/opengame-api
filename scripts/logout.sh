#!/bin/sh
curl -X POST http://0.0.0.0:3000/auth/logout \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $1" \
