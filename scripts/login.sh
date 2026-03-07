#!/bin/sh
curl -X POST http://0.0.0.0:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
        "username": "alice",
        "password": "supersecret"
      }'
