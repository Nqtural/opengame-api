#!/bin/sh
curl -X POST http://0.0.0.0:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
        "username": "alice",
        "email": "alice@example.com",
        "password": "supersecret"
      }'
