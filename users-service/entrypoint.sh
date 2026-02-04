#!/bin/bash
set -e
./users-migration up -u "$DATABASE_URL"
exec ./users-service
