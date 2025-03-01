# ORVRM - Open Source Rust Vehicle Route Machine

ORVRM is a vehicle routing optimization system built in Rust that uses OSRM (Open Source Routing Machine) for routing calculations. It provides a REST API for optimizing vehicle routes with various constraints.

## Features

- Vehicle routing optimization with capacity constraints
- Support for time windows
- Integration with OSRM for accurate routing
- REST API for easy integration
- Support for predefined routes

## Requirements

- Rust 1.70 or higher
- OSRM server (can be run locally or accessed remotely)

## Installation

### 1. Clone the repository

```bashv
git clone https://github.com/maartenbode/orvrm.git
cd orvrm
```

### 2. Build the project

```bash
cargo build --release
```

### 3. Run the server

```bash
./target/release/orvrm
```

By default, the server will listen on `127.0.0.1:8080`.

## Configuration

ORVRM can be configured using environment variables or configuration files. Create a `config` directory and add configuration files:

- `config/default.toml` - Default configuration
- `config/development.toml` - Development configuration
- `config/production.toml` - Production configuration

Example configuration:

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[osrm]
base_url = "http://localhost:5000"
default_profile = "car"
timeout_seconds = 30

[routing]
default_max_time = 30
default_threads = 4
```

Environment variables can also be used to override configuration:

```bash
APP__SERVER__HOST=0.0.0.0 APP__SERVER__PORT=8080 ./target/release/orvrm
```

## API Usage

### Optimize Routes

**Endpoint:** `POST /api/optimize`

**Request Body:**

```json
{
  "vehicles": [
    {
      "id": 1,
      "start": [6.0857, 52.5169],
      "end": [6.0857, 52.5169],
      "capacity": [4],
      "time_window": [1600416000, 1600426800]
    }
  ],
  "jobs": [
    {
      "id": 1,
      "location": [5.4174, 52.1853],
      "service": 300,
      "delivery": [2]
    },
    {
      "id": 2,
      "location": [5.7325, 52.2846],
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
```

**Response:**

```json
{
  "summary": {
    "cost": 7377.0,
    "distance": 141294,
    "duration": 7377,
    "routes": 1,
    "unassigned": 0,
    "computing_time": 41
  },
  "routes": [
    {
      "vehicle_id": 1,
      "route": [2, 1],
      "steps": [
        {
          "type": "start",
          "service_after": 0,
          "location": [6.0857, 52.5169],
          "arrival_time": 0,
          "departure_time": 0
        },
        {
          "type": "job",
          "id": 2,
          "location": [5.7325, 52.2846],
          "service": 300,
          "arrival_time": 2195,
          "departure_time": 2495
        },
        {
          "type": "job",
          "id": 1,
          "location": [5.4174, 52.1853],
          "service": 300,
          "arrival_time": 4413,
          "departure_time": 4713
        },
        {
          "type": "end",
          "location": [6.0857, 52.5169],
          "arrival_time": 7975,
          "departure_time": 7975
        }
      ],
      "distance": 141294,
      "duration": 7377,
      "arrival_times": [0, 2195, 4413, 7975],
      "departure_times": [0, 2495, 4713, 7975],
      "load_profile": [],
      "polyline": "encoded-polyline-for-vehicle-1"
    }
  ],
  "geometry": ["encoded-polyline-for-vehicle-1"]
}
```

### Health Check

**Endpoint:** `GET /api/health`

**Response:**

```json
{
  "status": "ok",
  "version": "0.1.0"
}
```
