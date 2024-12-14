User Service
-----
[![Go pipeline](https://github.com/MRdyRy/Ticket-Microservice/actions/workflows/go.yml/badge.svg)](https://github.com/MRdyRy/Ticket-Microservice/actions/workflows/go.yml)
![Go](https://badge.ttsalpha.com/api?label=Go&status=1.23&color=888888&labelColor=4CC3FF) 
![Docker](https://badge.ttsalpha.com/api?label=Docker&labelColor=3800FF)
![postgres](https://badge.ttsalpha.com/api?label=postgres&labelColor=7E7893)
![redis](https://badge.ttsalpha.com/api?label=redis&labelColor=F02E2E)
![kafka](https://badge.ttsalpha.com/api?label=kafka&labelColor=010101)

build :
```bash
go build .
```

running
```bash
go run .
```

running with docker compose :
```bash
cd ../infrastructure
docker-compose -d -f common.yml -f user-infra.yml up
```
