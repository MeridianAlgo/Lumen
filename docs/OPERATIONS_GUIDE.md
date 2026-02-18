# Lumina Chain Operations Guide

## Table of Contents
1. [System Requirements](#system-requirements)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Node Operations](#node-operations)
5. [Monitoring](#monitoring)
6. [Maintenance](#maintenance)
7. [Troubleshooting](#troubleshooting)
8. [Security](#security)
9. [Backup and Recovery](#backup-and-recovery)
10. [Performance Tuning](#performance-tuning)

## System Requirements

### Minimum Requirements
- **CPU**: 4+ cores (8+ recommended)
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 100GB+ SSD (NVMe recommended)
- **Network**: 100Mbps+ dedicated connection
- **OS**: Ubuntu 20.04+ or RHEL 8+

### Recommended Production Setup
- **CPU**: 8+ cores (Intel Xeon or AMD EPYC)
- **RAM**: 32GB minimum, 64GB recommended
- **Storage**: 1TB NVMe SSD
- **Network**: 1Gbps+ dedicated connection
- **Redundancy**: 3-5 node cluster

## Installation

### Prerequisites
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y build-essential clang cmake pkg-config libssl-dev

# RHEL/CentOS
sudo yum groupinstall "Development Tools"
sudo yum install openssl-devel
```

### Installation Methods

#### Method 1: Binary Installation
```bash
# Download latest release
wget https://github.com/luminachain/lumina/releases/latest/lumina-node
chmod +x lumina-node
sudo mv lumina-node /usr/local/bin/
```

#### Method 2: Build from Source
```bash
# Clone repository
git clone https://github.com/luminachain/lumina.git
cd lumina

# Build in release mode
cargo build --release --features "rocksdb,zk-snarks"

# Install binary
sudo cp target/release/lumina-node /usr/local/bin/
```

#### Method 3: Docker
```bash
# Pull Docker image
docker pull luminachain/node:latest

# Or build from source
docker build -t luminachain/node:latest .
```

## Configuration

### Main Configuration File
Create `config.toml`:

```toml
[network]
# Network identifier
network_id = "lumina-mainnet-1"

# P2P configuration
p2p_listen_addr = "/ip4/0.0.0.0/tcp/26656"
rpc_listen_addr = "0.0.0.0:26657"
p2p_seeds = "seed1.luminachain.com:26656,seed2.luminachain.com:26656"

[consensus]
timeout_propose = "3s"
timeout_commit = "1s"
timeout_prevote = "1s"
timeout_precommit = "1s"

[execution]
max_gas = 10000000
max_tx_size = 1048576  # 1MB
max_block_size = 10485760  # 10MB

[state]
snapshot_interval = 10000  # blocks
pruning = "everything"  # "nothing", "default", "everything"
pruning_interval = 10

[telemetry]
prometheus = true
prometheus_listen_addr = "0.0.0.0:26660"
```

### Environment Variables
```bash
export LUMINA_HOME=/var/lib/lumina
export LUMINA_LOG_LEVEL=info
export LUMINA_LOG_FORMAT=json
export RUST_LOG=info
```

## Node Operations

### Starting a Node

#### As a Service (Systemd)
```bash
# Create systemd service
sudo tee /etc/systemd/system/lumina.service << EOF
[Unit]
Description=Lumina Node
After=network.target

[Service]
Type=simple
User=lumina
Group=lumina
WorkingDirectory=/var/lib/lumina
ExecStart=/usr/local/bin/lumina-node start --home /var/lib/lumina
Restart=always
RestartSec=3
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable lumina
sudo systemctl start lumina
```

#### Docker Compose
```yaml
version: '3.8'
services:
  lumina-node:
    image: luminachain/node:latest
    container_name: lumina-node
    restart: unless-stopped
    ports:
      - "26656:26656"  # P2P
      - "26657:26657"  # RPC
      - "26660:26660"  # Metrics
    volumes:
      - ./data:/root/.lumina
      - ./config:/config
    command: start --home /root/.lumina
    environment:
      - MONIKER=my-validator
      - CHAIN_ID=lumina-1
    networks:
      - lumina-network

networks:
  lumina-network:
    driver: bridge
```

### Node Types

#### Validator Node
```bash
# Initialize node
lumina init my-validator --chain-id=lumina-1

# Add genesis account
lumina add-genesis-account $(lumina keys show validator -a) 1000000000ulum

# Create validator
lumina tx staking create-validator \
  --amount=1000000ulum \
  --pubkey=$(lumina tendermint show-validator) \
  --moniker="My Validator" \
  --commission-rate="0.10" \
  --commission-max-rate="0.20" \
  --commission-max-change-rate="0.01" \
  --min-self-delegation="1" \
  --from=validator
```

#### Full Node
```bash
# Initialize with genesis
lumina init my-full-node --chain-id=lumina-1

# Add persistent peers
peers="$(curl -s https://rpc.luminachain.com/peers.txt)"
sed -i "s/persistent_peers = \"\"/persistent_peers = \"$peers\"/" ~/.lumina/config/config.toml
```

#### Archive Node
```bash
# Enable archive mode
lumina start --pruning nothing --pruning-keep-recent 0
```

## Monitoring

### Prometheus Configuration
```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'lumina'
    static_configs:
      - targets: ['localhost:26660']
    metrics_path: /metrics
    scrape_interval: 15s
```

### Key Metrics to Monitor
```bash
# Node health
curl http://localhost:26657/health

# Node info
curl http://localhost:26657/status

# Network info
curl http://localhost:26657/net_info

# Validator set
curl http://localhost:26657/validators
```

### Grafana Dashboard
Import dashboard template:
```json
{
  "dashboard": {
    "panels": [
      {
        "title": "Block Height",
        "targets": [{
          "expr": "lumina_consensus_height",
          "legendFormat": "Block Height"
        }]
      }
    ]
  }
}
```

## Maintenance

### Regular Maintenance Tasks

#### Daily
1. Check node sync status
2. Monitor disk usage
3. Check validator signing status
4. Review logs for errors

#### Weekly
1. Backup node state
2. Update software
3. Review security patches
4. Check disk space

#### Monthly
1. Performance review
2. Security audit
3. Backup verification
4. Capacity planning

### Backup and Recovery

#### Automated Backups
```bash
#!/bin/bash
# backup.sh
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/backup/lumina"
mkdir -p $BACKUP_DIR

# Stop node for consistent backup
systemctl stop lumina

# Backup data
tar -czf $BACKUP_DIR/lumina_$DATE.tar.gz \
  ~/.lumina/data \
  ~/.lumina/config \
  ~/.lumina/wasm

# Restart node
systemctl start lumina

# Upload to cloud (optional)
# aws s3 cp $BACKUP_DIR/lumina_$DATE.tar.gz s3://my-backups/
```

#### Recovery
```bash
# Stop node
systemctl stop lumina

# Restore from backup
tar -xzf lumina_backup.tar.gz -C ~/.lumina

# Verify data integrity
lumina validate-genesis

# Start node
systemctl start lumina
```

## Performance Tuning

### Database Optimization
```bash
# RocksDB tuning
export ROCKSDB_MAX_OPEN_FILES=1000
export ROCKSDB_MAX_BACKGROUND_JOBS=4
export ROCKSDB_MAX_SUBCOMPACTIONS=4
```

### Memory Optimization
```toml
# config.toml
[storage]
# Cache size in MB
cache_size = 1024  # 1GB
max_open_files = 1000
```

### Network Optimization
```toml
[p2p]
max_num_inbound_peers = 40
max_num_outbound_peers = 20
flush_throttle_timeout = "100ms"
```

## Security

### Firewall Configuration
```bash
# Allow only necessary ports
sudo ufw default deny incoming
sudo ufw allow 26656  # P2P
sudo ufw allow 26657  # RPC
sudo ufw allow 26660  # Metrics
sudo ufw enable
```

### Key Management
```bash
# Create new key
lumina keys add validator --keyring-backend file

# Export/Import
lumina keys export validator --unarmored-hex --unsafe
lumina keys import validator key.asc

# Multi-sig setup
lumina tx multisign transaction.json validator1 validator2
```

### Monitoring Commands

#### Check Node Health
```bash
# Check sync status
curl http://localhost:26657/status | jq .result.sync_info

# Check validator status
lumina query staking validator $(lumina keys show validator -a)

# Check network
curl http://localhost:26657/net_info | jq .result.n_peers
```

#### Performance Monitoring
```bash
# Monitor disk I/O
iostat -x 1

# Monitor memory
free -h

# Monitor network
iftop -i eth0

# Check logs in real-time
journalctl -u lumina -f
```

## Troubleshooting

### Common Issues

#### Node Not Syncing
```bash
# Check if node is catching up
curl http://localhost:26657/status | jq .result.sync_info

# Check peer connections
curl http://localhost:26657/net_info | jq .result.n_peers

# Reset and resync (if needed)
lumina unsafe-reset-all
```

#### High Memory Usage
```bash
# Check memory usage
ps aux | grep lumina
free -h

# Adjust memory limits
export GOGC=100  # Garbage collection
export GOMAXPROCS=4
```

#### Connection Issues
```bash
# Check if ports are open
netstat -tulpn | grep 26656
ss -tulpn | grep 26656

# Check firewall
sudo ufw status
```

### Log Analysis
```bash
# Follow logs
journalctl -u lumina -f

# Search for errors
journalctl -u lumina --since "1 hour ago" | grep -i error

# Monitor specific log level
RUST_LOG=info lumina start
```

## Backup and Recovery Procedures

### Automated Backups
```bash
#!/bin/bash
# backup_lumina.sh
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/backup/lumina"
NODE_HOME="$HOME/.lumina"

# Stop node if running
systemctl stop lumina

# Create backup
tar -czf "$BACKUP_DIR/lumina_$DATE.tar.gz" \
  --exclude="*.wasm" \
  --exclude="*.wasm.gz" \
  $NODE_HOME/data \
  $NODE_HOME/config

# Restart node
systemctl start lumina

# Keep only last 7 days
find $BACKUP_DIR -name "lumina_*.tar.gz" -mtime +7 -delete
```

### Disaster Recovery
```bash
# 1. Stop the node
systemctl stop lumina

# 2. Restore from backup
tar -xzf lumina_backup.tar.gz -C ~/

# 3. Verify data integrity
lumina validate-genesis

# 4. Start with recovery mode
lumina start --recover
```

## Performance Tuning

### Database Optimization
```toml
[storage]
# RocksDB options
max_open_files = 1000
keep_log_file_num = 10
max_log_file_size = "100MB"
```

### Memory Management
```bash
# JVM-like memory settings for Go
export GOMEMLIMIT=4G
export GOMAXPROCS=8
export GODEBUG=madvdontneed=1
```

### Network Tuning
```toml
[p2p]
send_rate = 5120000  # 5 MB/s
recv_rate = 5120000
max_packet_msg_payload_size = 1024
```

## Monitoring and Alerts

### Alert Rules
```yaml
groups:
  - name: lumina
    rules:
    - alert: NodeDown
      expr: up{job="lumina"} == 0
      for: 5m
      labels:
        severity: critical
      annotations:
        summary: "Lumina node is down"
        
    - alert: HighMemoryUsage
      expr: process_resident_memory_bytes / 1024 / 1024 > 4000
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: "High memory usage"
```

### Health Checks
```bash
#!/bin/bash
# health_check.sh

# Check if node is synced
SYNC_STATUS=$(curl -s http://localhost:26657/status | jq -r .result.sync_info.syncing)
if [ "$SYNC_STATUS" != "false" ]; then
    echo "Node is not synced"
    exit 1
fi

# Check disk space
DISK_USAGE=$(df / | tail -1 | awk '{print $5}' | sed 's/%//')
if [ $DISK_USAGE -gt 90 ]; then
    echo "Disk usage critical: $DISK_USAGE%"
    exit 1
fi
```

## Security Best Practices

### 1. Key Management
- Use hardware wallets for validators
- Implement multi-sig for treasury
- Regular key rotation

### 2. Network Security
- Use VPN/SSH tunnels for RPC endpoints
- Implement rate limiting
- Use TLS for RPC endpoints

### 3. Regular Audits
- Monthly security audits
- Penetration testing
- Dependency scanning

### 4. Access Control
```bash
# Restrict RPC access
lumina start --rpc.laddr "tcp://127.0.0.1:26657"
```

## Performance Benchmarks

### Expected Performance
- **TPS**: 8,000+ transactions per second
- **Block Time**: 2 seconds
- **Finality**: 1-2 seconds
- **Storage**: 1TB+ per year

### Monitoring Dashboard
Create Grafana dashboard with:
1. Block production rate
2. Transaction throughput
3. Validator performance
4. Network health
5. Storage usage

## Emergency Procedures

### Node Recovery
```bash
# 1. Stop the node
systemctl stop lumina

# 2. Backup current state
cp -r ~/.lumina/data ~/.lumina/data.backup

# 3. Reset to safe state
lumina unsafe-reset-all

# 4. Restore from snapshot or sync from genesis
lumina start --p2p.persistent_peers "peer1,peer2,peer3"
```

### Validator Slashing
If slashed:
1. Check slashing reason
2. Unbond and redelegate
3. Monitor for 21 days
4. Re-enter validator set

## Support and Resources

### Official Resources
- Documentation: docs.luminachain.com
- Status: status.luminachain.com
- Support: support@luminachain.com

### Community
- Discord: discord.gg/lumina
- Forum: forum.luminachain.com
- GitHub: github.com/luminachain

### Emergency Contacts
- Security: security@luminachain.com
- Validator Support: validators@luminachain.com
- Technical Support: support@luminachain.com

---

*Last Updated: February 2026*  
*Version: 2.1.0*