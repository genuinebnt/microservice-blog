#!/bin/bash
set -e
./notification-migration up -u "$DATABASE_URL"
exec ./notification-service
