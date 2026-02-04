#!/bin/bash
set -e
./posts-migration up -u "$DATABASE_URL"
exec ./posts-service
