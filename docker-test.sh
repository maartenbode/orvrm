#!/bin/bash

# Check if Docker Compose is running
if ! docker-compose ps | grep -q "orvrm.*Up"; then
  echo "ORVRM is not running. Please start it with 'docker-compose up -d'"
  exit 1
fi

# Test data
cat << 'EOF' > /tmp/orvrm-test.json
{
  "vehicles": [
    {
      "id": 1,
      "start": [2.35044, 48.71764],
      "end": [2.35044, 48.71764],
      "capacity": [4],
      "time_window": [1600416000, 1600426800]
    }
  ],
  "jobs": [
    {
      "id": 1,
      "location": [1.98935, 48.701],
      "service": 300,
      "delivery": [2]
    },
    {
      "id": 2,
      "location": [2.03655, 48.61128],
      "service": 300,
      "delivery": [2]
    }
  ],
  "options": {
    "max_time": 30,
    "threads": 4,
    "geometry": true
  }
}
EOF

# Send request to API
echo "Sending test request to ORVRM API..."
curl -X POST -H "Content-Type: application/json" -d @/tmp/orvrm-test.json http://localhost:8080/api/optimize

# Clean up
rm /tmp/orvrm-test.json 