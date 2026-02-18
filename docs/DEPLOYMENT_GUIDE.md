# Lumina Chain Deployment Guide

## Table of Contents
1. [Deployment Overview](#deployment-overview)
2. [Prerequisites](#prerequisites)
3. [Single Node Deployment](#single-node-deployment)
4. [Validator Node Deployment](#validator-node-deployment)
5. [Multi-Node Cluster](#multi-node-cluster)
6. [Docker Deployment](#docker-deployment)
7. [Kubernetes Deployment](#kubernetes-deployment)
8. [Cloud Deployment](#cloud-deployment)
9. [Monitoring Setup](#monitoring-setup)
10. [Backup and Recovery](#backup-and-recovery)
11. [Scaling](#scaling)
12. [Maintenance](#maintenance)

## Deployment Overview

Lumina Chain supports multiple deployment scenarios from single-node development setups to large-scale production clusters.

### Deployment Scenarios

| Scenario | Nodes | Use Case | Complexity |
|----------|-------|----------|------------|
| Development | 1 | Local testing | Low |
| Testnet | 3-5 | Staging/testing | Medium |
| Production | 7+ | Mainnet | High |
| Enterprise | 21+ | Large-scale | Very High |

## Prerequisites

### System Requirements

#### Minimum Requirements
- **CPU**: 4 cores (x86_64)
- **RAM**: 8GB
- **Storage**: 100GB SSD
- **Network**: 100Mbps
- **OS**: Ubuntu 20.04+, RHEL 8+, or macOS 12+

#### Recommended Production
- **CPU**: 8+ cores (Intel Xeon/AMD EPYC)
- **RAM**: 32GB+
- **Storage**: 1TB NVMe SSD
- **Network**: 1Gbps+
- **OS**: Ubuntu 22.04 LTS

### Software Dependencies

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  clang \
  cmake \
  pkg-config \
  libssl-dev \
  git \
  curl \
  wget \
  jq \
  tmux \
  htop

# RHEL/CentOS
sudo yum groupinstall "Development Tools"
sudo yum install openssl-devel git curl wget jq tmux htop

# macOS
brew install cmake pkg-config openssl git curl wget jq tmux htop
```

### Network Requirements

| Port | Protocol | Purpose | Access |
|------|----------|---------|--------|
| 26656 | TCP | P2P | Public |
| 26657 | TCP | RPC | Restricted |
| 26660 | TCP | Metrics | Internal |
| 9090 | TCP | gRPC | Restricted |
| 1317 | TCP | REST API | Public/Restricted |

## Single Node Deployment

### Step 1: Install Lumina

```bash
# Method 1: Binary installation
wget https://github.com/luminachain/lumina/releases/latest/lumina-node
chmod +x lumina-node
sudo mv lumina-node /usr/local/bin/

# Method 2: Build from source
git clone https://github.com/luminachain/lumina.git
cd lumina
cargo build --release
sudo cp target/release/lumina-node /usr/local/bin/
```

### Step 2: Initialize Node

```bash
# Set node name
MONIKER="my-node"

# Initialize node
lumina init $MONIKER --chain-id=lumina-1

# Create or import wallet
lumina keys add my-wallet
# OR import existing
lumina keys import my-wallet --recover
```

### Step 3: Configure Genesis

```bash
# Download genesis file
wget https://raw.githubusercontent.com/luminachain/mainnet/master/genesis.json
mv genesis.json ~/.lumina/config/genesis.json

# Validate genesis
lumina validate-genesis
```

### Step 4: Configure Node

```bash
# Edit config.toml
nano ~/.lumina/config/config.toml

# Set persistent peers
persistent_peers = "node1.luminachain.com:26656,node2.luminachain.com:26656"

# Set seeds
seeds = "seed1.luminachain.com:26656,seed2.luminachain.com:26656"
```

### Step 5: Start Node

```bash
# Start as service
sudo tee /etc/systemd/system/lumina.service << EOF
[Unit]
Description=Lumina Node
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME
ExecStart=/usr/local/bin/lumina-node start
Restart=always
RestartSec=3
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable lumina
sudo systemctl start lumina

# Check status
sudo systemctl status lumina
journalctl -u lumina -f
```

## Validator Node Deployment

### Step 1: Set Up Validator

```bash
# Create validator key
lumina keys add validator --keyring-backend file

# Add genesis account
lumina add-genesis-account $(lumina keys show validator -a) 1000000000ulum

# Create validator transaction
lumina gentx validator 1000000ulum \
  --chain-id=lumina-1 \
  --moniker="My Validator" \
  --commission-rate="0.10" \
  --commission-max-rate="0.20" \
  --commission-max-change-rate="0.01" \
  --min-self-delegation="1"

# Collect gentxs
lumina collect-gentxs
```

### Step 2: Configure Validator

```toml
# ~/.lumina/config/config.toml
[consensus]
timeout_propose = "3s"
timeout_propose_delta = "500ms"
timeout_prevote = "1s"
timeout_prevote_delta = "500ms"
timeout_precommit = "1s"
timeout_precommit_delta = "500ms"
timeout_commit = "5s"

# Enable validator mode
mode = "validator"
```

### Step 3: Security Configuration

```bash
# Create dedicated user
sudo useradd -r -s /bin/false lumina
sudo chown -R lumina:lumina ~/.lumina

# Configure firewall
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow 26656/tcp  # P2P
sudo ufw allow 22/tcp     # SSH
sudo ufw enable
```

### Step 4: Start Validator

```bash
# Update service file for validator user
sudo sed -i 's/User=.*/User=lumina/' /etc/systemd/system/lumina.service
sudo sed -i 's/Group=.*/Group=lumina/' /etc/systemd/system/lumina.service

# Restart service
sudo systemctl daemon-reload
sudo systemctl restart lumina
```

## Multi-Node Cluster

### Architecture

```
┌─────────────────────────────────────┐
│         Load Balancer               │
├─────────────────────────────────────┤
│  Validator 1  │  Validator 2  │ ...│
├─────────────────────────────────────┤
│  Full Node 1  │  Full Node 2  │ ...│
├─────────────────────────────────────┤
│  Archive Node │  Sentry Node  │ ...│
└─────────────────────────────────────┘
```

### Deployment Script

```bash
#!/bin/bash
# deploy-cluster.sh

NODES=7
CHAIN_ID="lumina-1"
MONIKER_PREFIX="validator"

# Generate genesis
lumina init $MONIKER_PREFIX-1 --chain-id=$CHAIN_ID

# Create validators
for i in $(seq 1 $NODES); do
  MONIKER="$MONIKER_PREFIX-$i"
  
  # Initialize node
  lumina init $MONIKER --chain-id=$CHAIN_ID
  
  # Create validator key
  lumina keys add $MONIKER --keyring-backend file
  
  # Add to genesis
  lumina add-genesis-account $(lumina keys show $MONIKER -a) 1000000000ulum
  
  # Create gentx
  lumina gentx $MONIKER 1000000ulum \
    --chain-id=$CHAIN_ID \
    --moniker=$MONIKER
done

# Collect gentxs
lumina collect-gentxs

# Distribute genesis to all nodes
for i in $(seq 1 $NODES); do
  scp genesis.json node-$i:~/.lumina/config/
done
```

### Configuration Management

```yaml
# ansible/inventory.yml
all:
  hosts:
    validator-1:
      ansible_host: 10.0.1.1
      node_type: validator
    validator-2:
      ansible_host: 10.0.1.2
      node_type: validator
    full-1:
      ansible_host: 10.0.2.1
      node_type: full
    archive-1:
      ansible_host: 10.0.3.1
      node_type: archive
```

## Docker Deployment

### Docker Compose

```yaml
# docker-compose.yml
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
      - "1317:1317"    # REST API
      - "9090:9090"    # gRPC
    volumes:
      - ./data:/root/.lumina
      - ./config:/config
    environment:
      - MONIKER=my-validator
      - CHAIN_ID=lumina-1
      - LOG_LEVEL=info
    command: start --home /root/.lumina
    networks:
      - lumina-network

  prometheus:
    image: prom/prometheus:latest
    container_name: prometheus
    ports:
      - "9091:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'
      - '--storage.tsdb.retention.time=200h'
      - '--web.enable-lifecycle'
    networks:
      - lumina-network

  grafana:
    image: grafana/grafana:latest
    container_name: grafana
    ports:
      - "3001:3000"
    volumes:
      - grafana_data:/var/lib/grafana
      - ./grafana/dashboards:/etc/grafana/provisioning/dashboards
      - ./grafana/datasources:/etc/grafana/provisioning/datasources
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    networks:
      - lumina-network

networks:
  lumina-network:
    driver: bridge

volumes:
  prometheus_data:
  grafana_data:
```

### Docker Swarm

```yaml
# docker-stack.yml
version: '3.8'

services:
  lumina-validator:
    image: luminachain/node:latest
    deploy:
      mode: replicated
      replicas: 7
      placement:
        constraints:
          - node.role == manager
      resources:
        limits:
          cpus: '4'
          memory: 16G
        reservations:
          cpus: '2'
          memory: 8G
    ports:
      - target: 26656
        published: 26656
        protocol: tcp
        mode: host
    volumes:
      - type: volume
        source: lumina-data
        target: /root/.lumina
    networks:
      - lumina-network
    command: start --validator

  lumina-full:
    image: luminachain/node:latest
    deploy:
      mode: replicated
      replicas: 3
      placement:
        constraints:
          - node.role == worker
    ports:
      - target: 26657
        published: 26657
    volumes:
      - type: volume
        source: lumina-data
        target: /root/.lumina
    networks:
      - lumina-network
    command: start --full-node

networks:
  lumina-network:
    driver: overlay
    attachable: true

volumes:
  lumina-data:
    driver: local
```

## Kubernetes Deployment

### Namespace Configuration

```yaml
# namespace.yml
apiVersion: v1
kind: Namespace
metadata:
  name: lumina
  labels:
    name: lumina
```

### ConfigMap for Configuration

```yaml
# configmap.yml
apiVersion: v1
kind: ConfigMap
metadata:
  name: lumina-config
  namespace: lumina
data:
  config.toml: |
    [p2p]
    laddr = "tcp://0.0.0.0:26656"
    persistent_peers = "validator-0.lumina:26656,validator-1.lumina:26656"
    
    [rpc]
    laddr = "tcp://0.0.0.0:26657"
    
    [consensus]
    timeout_propose = "3s"
    timeout_commit = "5s"
```

### StatefulSet for Validators

```yaml
# statefulset.yml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: lumina-validator
  namespace: lumina
spec:
  serviceName: lumina-validator
  replicas: 7
  selector:
    matchLabels:
      app: lumina
      role: validator
  template:
    metadata:
      labels:
        app: lumina
        role: validator
    spec:
      securityContext:
        runAsUser: 1000
        runAsGroup: 1000
        fsGroup: 1000
      containers:
      - name: lumina-node
        image: luminachain/node:latest
        imagePullPolicy: Always
        ports:
        - containerPort: 26656
          name: p2p
        - containerPort: 26657
          name: rpc
        - containerPort: 26660
          name: metrics
        volumeMounts:
        - name: data
          mountPath: /root/.lumina
        - name: config
          mountPath: /config
        env:
        - name: MONIKER
          valueFrom:
            fieldRef:
              fieldPath: metadata.name
        - name: CHAIN_ID
          value: "lumina-1"
        resources:
          requests:
            memory: "8Gi"
            cpu: "2"
          limits:
            memory: "16Gi"
            cpu: "4"
        livenessProbe:
          httpGet:
            path: /health
            port: 26657
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /status
            port: 26657
          initialDelaySeconds: 5
          periodSeconds: 5
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 1Ti
      storageClassName: fast-ssd
```

### Service Configuration

```yaml
# service.yml
apiVersion: v1
kind: Service
metadata:
  name: lumina-validator
  namespace: lumina
  labels:
    app: lumina
    role: validator
spec:
  clusterIP: None
  ports:
  - port: 26656
    name: p2p
  - port: 26657
    name: rpc
  - port: 26660
    name: metrics
  selector:
    app: lumina
    role: validator
```

### Ingress for API Access

```yaml
# ingress.yml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: lumina-api
  namespace: lumina
  annotations:
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    nginx.ingress.kubernetes.io/backend-protocol: "HTTP"
spec:
  tls:
  - hosts:
    - api.luminachain.com
    secretName: lumina-tls
  rules:
  - host: api.luminachain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: lumina-validator
            port:
              number: 26657
```

## Cloud Deployment

### AWS Deployment

#### CloudFormation Template
```yaml
# cloudformation.yml
AWSTemplateFormatVersion: '2010-09-09'
Description: Lumina Chain Validator Cluster

Parameters:
  InstanceType:
    Type: String
    Default: m5.2xlarge
    AllowedValues:
      - m5.large
      - m5.xlarge
      - m5.2xlarge
      - m5.4xlarge
  
  NodeCount:
    Type: Number
    Default: 7
    MinValue: 3
    MaxValue: 21
  
  VolumeSize:
    Type: Number
    Default: 1000
    MinValue: 100
    MaxValue: 10000

Resources:
  LuminaSecurityGroup:
    Type: AWS::EC2::SecurityGroup
    Properties:
      GroupDescription: Security group for Lumina nodes
      SecurityGroupIngress:
        - IpProtocol: tcp
          FromPort: 26656
          ToPort: 26656
          CidrIp: 0.0.0.0/0
        - IpProtocol: tcp
          FromPort: 22
          ToPort: 22
          CidrIp: 10.0.0.0/8

  LuminaLaunchTemplate:
    Type: AWS::EC2::LaunchTemplate
    Properties:
      LaunchTemplateData:
        InstanceType: !Ref InstanceType
        ImageId: ami-0c55b159cbfafe1f0  # Ubuntu 22.04
        BlockDeviceMappings:
          - DeviceName: /dev/sda1
            Ebs:
              VolumeSize: !Ref VolumeSize
              VolumeType: gp3
              Iops: 3000
              Throughput: 125
        UserData:
          Fn::Base64: !Sub |
            #!/bin/bash
            apt-get update
            apt-get install -y docker.io
            docker run -d \
              -p 26656:26656 \
              -p 26657:26657 \
              -v /data:/root/.lumina \
              luminachain/node:latest \
              start --validator

  LuminaAutoScalingGroup:
    Type: AWS::AutoScaling::AutoScalingGroup
    Properties:
      LaunchTemplate:
        LaunchTemplateId: !Ref LuminaLaunchTemplate
        Version: !GetAtt LuminaLaunchTemplate.LatestVersionNumber
      MinSize: !Ref NodeCount
      MaxSize: !Ref NodeCount
      DesiredCapacity: !Ref NodeCount
      VPCZoneIdentifier:
        - subnet-12345678
        - subnet-87654321
      HealthCheckType: EC2
      HealthCheckGracePeriod: 300
      Tags:
        - Key: Name
          Value: Lumina-Validator
          PropagateAtLaunch: true
```

### Google Cloud Deployment

#### Deployment Manager Template
```yaml
# deployment-manager.yml
resources:
- name: lumina-validator-group
  type: compute.v1.instanceGroupManager
  properties:
    baseInstanceName: lumina-validator
    instanceTemplate: $(ref.lumina-template.selfLink)
    targetSize: 7
    zone: us-central1-a

- name: lumina-template
  type: compute.v1.instanceTemplate
  properties:
    properties:
      machineType: n2-standard-8
      disks:
      - deviceName: boot
        type: PERSISTENT
        boot: true
        autoDelete: true
        initializeParams:
          sourceImage: projects/ubuntu-os-cloud/global/images/ubuntu-2204-jammy-v20240201
          diskSizeGb: 1000
          diskType: pd-ssd
      networkInterfaces:
      - network: global/networks/default
        accessConfigs:
        - name: External NAT
          type: ONE_TO_ONE_NAT
      metadata:
        items:
        - key: startup-script
          value: |
            #!/bin/bash
            apt-get update
            apt-get install -y docker.io
            docker run -d \
              -p 26656:26656 \
              -p 26657:26657 \
              -v /data:/root/.lumina \
              luminachain/node:latest \
              start --validator
```

## Monitoring Setup

### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'lumina'
    static_configs:
      - targets: ['lumina-validator-0:26660', 'lumina-validator-1:26660']
    metrics_path: /metrics
    scrape_interval: 5s

  - job_name: 'node-exporter'
    static_configs:
      - targets: ['node-exporter:9100']

  - job_name: 'cadvisor'
    static_configs:
      - targets: ['cadvisor:8080']
```

### Grafana Dashboards

```json
{
  "dashboard": {
    "title": "Lumina Chain Monitoring",
    "panels": [
      {
        "title": "Block Height",
        "type": "graph",
        "targets": [{
          "expr": "lumina_consensus_height",
          "legendFormat": "{{instance}}"
        }]
      },
      {
        "title": "Validator Voting Power",
        "type": "stat",
        "targets": [{
          "expr": "lumina_validator_voting_power",
          "legendFormat": "{{validator}}"
        }]
      }
    ]
  }
}
```

## Backup and Recovery

### Automated Backup Script

```bash
#!/bin/bash
# backup.sh

BACKUP_DIR="/backup/lumina"
DATE=$(date +%Y%m%d_%H%M%S)
NODE_HOME="/root/.lumina"

# Create backup directory
mkdir -p $BACKUP_DIR

# Stop node for consistent backup
systemctl stop lumina

# Create backup
tar -czf $BACKUP_DIR/lumina_$DATE.tar.gz \
  --exclude="*.wasm" \
  --exclude="*.wasm.gz" \
  $NODE_HOME/data \
  $NODE_HOME/config \
  $NODE_HOME/wasm

# Start node
systemctl start lumina

# Upload to S3 (optional)
aws s3 cp $BACKUP_DIR/lumina_$DATE.tar.gz s3://lumina-backups/

# Cleanup old backups (keep 7 days)
find $BACKUP_DIR -name "lumina_*.tar.gz" -mtime +7 -delete
```

### Recovery Procedure

```bash
#!/bin/bash
# restore.sh

BACKUP_FILE="lumina_20260217_103000.tar.gz"
BACKUP_DIR="/backup/lumina"
NODE_HOME="/root/.lumina"

# Stop node
systemctl stop lumina

# Restore from backup
tar -xzf $BACKUP_DIR/$BACKUP_FILE -C /

# Verify data integrity
lumina validate-genesis

# Start node
systemctl start lumina

# Monitor recovery
journalctl -u lumina -f
```

## Scaling

### Horizontal Scaling

```bash
# Add new validator
lumina tx staking create-validator \
  --amount=1000000ulum \
  --pubkey=$(lumina tendermint show-validator) \
  --moniker="new-validator" \
  --commission-rate="0.10" \
  --from=new-validator
```

### Vertical Scaling

```bash
# Update node resources
# 1. Stop node
systemctl stop lumina

# 2. Update configuration
sed -i 's/max_open_files = .*/max_open_files = 5000/' ~/.lumina/config/config.toml

# 3. Increase system limits
echo "lumina soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "lumina hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# 4. Restart node
systemctl start lumina
```

## Maintenance

### Regular Maintenance Tasks

#### Daily
```bash
# Check node health
./health-check.sh

# Monitor disk space
df -h /data

# Check logs
journalctl -u lumina --since "24 hours ago" | grep -i error
```

#### Weekly
```bash
# Update software
cargo update
cargo build --release
systemctl restart lumina

# Backup
./backup.sh

# Security updates
sudo apt-get update
sudo apt-get upgrade
```

#### Monthly
```bash
# Performance review
./performance-review.sh

# Security audit
./security-audit.sh

# Capacity planning
./capacity-planning.sh
```

### Upgrade Procedure

```bash
# 1. Backup current state
./backup.sh

# 2. Stop node
systemctl stop lumina

# 3. Update binary
wget https://github.com/luminachain/lumina/releases/latest/lumina-node
chmod +x lumina-node
sudo mv lumina-node /usr/local/bin/

# 4. Migrate state if needed
lumina migrate

# 5. Start node
systemctl start lumina

# 6. Verify upgrade
lumina version
journalctl -u lumina --since "5 minutes ago"
```

## Troubleshooting

### Common Issues

#### Node Not Syncing
```bash
# Check sync status
curl http://localhost:26657/status | jq .result.sync_info

# Check peers
curl http://localhost:26657/net_info | jq .result.n_peers

# Reset and resync
lumina unsafe-reset-all
```

#### High Memory Usage
```bash
# Check memory usage
ps aux | grep lumina
free -h

# Adjust memory limits
export GOGC=100
export GOMEMLIMIT=4G
```

#### Connection Issues
```bash
# Check ports
netstat -tulpn | grep 26656

# Check firewall
sudo ufw status

# Test connectivity
nc -zv node.luminachain.com 26656
```

## Support

### Getting Help
- Documentation: docs.luminachain.com
- Support: support@luminachain.com
- Community: discord.gg/lumina
- Status: status.luminachain.com

### Emergency Contacts
- Security: security@luminachain.com
- Validator Support: validators@luminachain.com
- Technical Support: support@luminachain.com

---

*Last Updated: February 2026*  
*Version: 2.1.0*