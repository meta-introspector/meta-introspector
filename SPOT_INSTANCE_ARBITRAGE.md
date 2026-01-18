# Spot Instance Arbitrage for Nix Builds

## Concept

**Use prediction markets + spot pricing to buy compute exactly when and where needed!**

Markets predict:
1. Which flakes will be built tomorrow
2. Which regions will be active (follow-the-sun)
3. When peak demand occurs

Then:
- Buy spot instances in cheapest region
- Build predicted flakes just-in-time
- Terminate instances after build
- Profit from arbitrage

## Architecture

```
Prediction Markets
    ↓
Spot Price Monitor
    ↓
Build Scheduler
    ↓
Spot Instance Buyer
    ↓
Nix Builder
    ↓
Binary Cache
```

## Implementation

### 1. Spot Price Monitor

```python
#!/usr/bin/env python3
"""Monitor spot prices across regions and clouds"""

import boto3
import requests
from datetime import datetime

class SpotPriceMonitor:
    def __init__(self):
        self.ec2 = boto3.client('ec2')
        self.regions = [
            'ap-northeast-1',  # Tokyo
            'eu-west-1',       # Ireland
            'us-east-1',       # Virginia
        ]
    
    def get_spot_prices(self, instance_type='c6i.8xlarge'):
        """Get current spot prices across regions"""
        prices = {}
        
        for region in self.regions:
            ec2 = boto3.client('ec2', region_name=region)
            
            response = ec2.describe_spot_price_history(
                InstanceTypes=[instance_type],
                ProductDescriptions=['Linux/UNIX'],
                MaxResults=1
            )
            
            if response['SpotPriceHistory']:
                price = float(response['SpotPriceHistory'][0]['SpotPrice'])
                prices[region] = price
        
        return prices
    
    def find_cheapest_region(self, instance_type='c6i.8xlarge'):
        """Find cheapest region for spot instances"""
        prices = self.get_spot_prices(instance_type)
        cheapest = min(prices.items(), key=lambda x: x[1])
        return cheapest[0], cheapest[1]
    
    def predict_price_trend(self, region, hours=24):
        """Predict spot price trend"""
        ec2 = boto3.client('ec2', region_name=region)
        
        response = ec2.describe_spot_price_history(
            InstanceTypes=['c6i.8xlarge'],
            ProductDescriptions=['Linux/UNIX'],
            MaxResults=hours
        )
        
        prices = [float(p['SpotPrice']) for p in response['SpotPriceHistory']]
        
        # Simple trend: average of last N hours
        avg_price = sum(prices) / len(prices)
        current_price = prices[0]
        
        trend = 'rising' if current_price > avg_price else 'falling'
        return trend, avg_price

# Usage
monitor = SpotPriceMonitor()
cheapest_region, price = monitor.find_cheapest_region()
print(f"Cheapest: {cheapest_region} at ${price}/hour")
```

### 2. Build Scheduler

```python
class BuildScheduler:
    def __init__(self):
        self.market_client = PredictionMarketClient()
        self.spot_monitor = SpotPriceMonitor()
    
    def schedule_builds(self):
        """Schedule builds based on markets + spot prices"""
        
        # Get predictions for tomorrow
        predictions = self.market_client.get_predictions('tomorrow')
        
        # Group by expected build time
        builds_by_hour = {}
        for pred in predictions:
            if pred['probability'] > 0.7:
                # Predict when this flake will be built
                hour = self.predict_build_time(pred)
                if hour not in builds_by_hour:
                    builds_by_hour[hour] = []
                builds_by_hour[hour].append(pred)
        
        # Schedule spot instances
        schedule = []
        for hour, builds in builds_by_hour.items():
            # Find cheapest region at this hour
            region, price = self.find_cheapest_region_at_time(hour)
            
            # Calculate required capacity
            total_cpu_hours = sum(b['estimated_cpu_hours'] for b in builds)
            instances_needed = self.calculate_instances(total_cpu_hours)
            
            schedule.append({
                'hour': hour,
                'region': region,
                'spot_price': price,
                'instances': instances_needed,
                'builds': builds,
                'total_cost': price * instances_needed
            })
        
        return schedule
    
    def predict_build_time(self, prediction):
        """Predict when flake will be built based on timezone"""
        # If prediction is for Asia region, build at 9 AM JST
        if 'Asia' in prediction.get('timezone', ''):
            return 9  # 9 AM local time
        elif 'Europe' in prediction.get('timezone', ''):
            return 9  # 9 AM local time
        else:
            return 9  # 9 AM local time
    
    def find_cheapest_region_at_time(self, hour):
        """Find cheapest region at specific hour"""
        # Query historical spot prices
        prices = self.spot_monitor.get_historical_prices(hour)
        cheapest = min(prices.items(), key=lambda x: x[1])
        return cheapest
    
    def calculate_instances(self, cpu_hours):
        """Calculate number of instances needed"""
        # c6i.8xlarge = 32 vCPUs
        # Build in 1 hour window
        instances = (cpu_hours / 32) + 1  # Round up
        return int(instances)
```

### 3. Spot Instance Buyer

```python
class SpotInstanceBuyer:
    def __init__(self):
        self.ec2_clients = {}
    
    def buy_spot_instances(self, region, count, max_price):
        """Buy spot instances in region"""
        if region not in self.ec2_clients:
            self.ec2_clients[region] = boto3.client('ec2', region_name=region)
        
        ec2 = self.ec2_clients[region]
        
        # Request spot instances
        response = ec2.request_spot_instances(
            InstanceCount=count,
            Type='one-time',
            LaunchSpecification={
                'ImageId': self.get_nix_ami(region),
                'InstanceType': 'c6i.8xlarge',
                'KeyName': 'nix-builder',
                'SecurityGroups': ['nix-builder-sg'],
                'UserData': self.get_user_data(),
            },
            SpotPrice=str(max_price * 1.1)  # Bid 10% above current
        )
        
        request_ids = [r['SpotInstanceRequestId'] for r in response['SpotInstanceRequests']]
        
        # Wait for fulfillment
        instances = self.wait_for_instances(ec2, request_ids)
        
        return instances
    
    def get_user_data(self):
        """User data script to setup Nix builder"""
        return '''#!/bin/bash
        # Install Nix
        curl -L https://nixos.org/nix/install | sh
        . /home/ec2-user/.nix-profile/etc/profile.d/nix.sh
        
        # Configure binary cache
        nix-env -iA nixpkgs.cachix
        cachix use meta-introspector
        
        # Start builder daemon
        nix-daemon &
        
        # Signal ready
        aws sns publish --topic-arn arn:aws:sns:region:account:nix-builder-ready \
            --message "$(hostname) ready"
        '''
    
    def wait_for_instances(self, ec2, request_ids, timeout=300):
        """Wait for spot requests to be fulfilled"""
        import time
        start = time.time()
        
        while time.time() - start < timeout:
            response = ec2.describe_spot_instance_requests(
                SpotInstanceRequestIds=request_ids
            )
            
            all_fulfilled = all(
                r['State'] == 'active' 
                for r in response['SpotInstanceRequests']
            )
            
            if all_fulfilled:
                instance_ids = [
                    r['InstanceId'] 
                    for r in response['SpotInstanceRequests']
                ]
                return instance_ids
            
            time.sleep(10)
        
        raise TimeoutError("Spot instances not fulfilled in time")
```

### 4. Just-in-Time Builder

```python
class JustInTimeBuilder:
    def __init__(self):
        self.scheduler = BuildScheduler()
        self.buyer = SpotInstanceBuyer()
    
    def build_on_schedule(self):
        """Execute scheduled builds on spot instances"""
        
        # Get build schedule
        schedule = self.scheduler.schedule_builds()
        
        for slot in schedule:
            hour = slot['hour']
            region = slot['region']
            builds = slot['builds']
            instances_needed = slot['instances']
            max_price = slot['spot_price']
            
            print(f"Hour {hour}: Building in {region}")
            print(f"  Spot price: ${max_price}/hour")
            print(f"  Instances: {instances_needed}")
            print(f"  Builds: {len(builds)}")
            
            # Wait until scheduled time
            self.wait_until(hour)
            
            # Buy spot instances
            instances = self.buyer.buy_spot_instances(
                region, 
                instances_needed, 
                max_price
            )
            
            # Distribute builds across instances
            builds_per_instance = len(builds) // len(instances)
            
            for i, instance_id in enumerate(instances):
                instance_builds = builds[i*builds_per_instance:(i+1)*builds_per_instance]
                self.execute_builds(instance_id, instance_builds)
            
            # Wait for completion
            self.wait_for_completion(instances)
            
            # Terminate instances
            self.terminate_instances(region, instances)
            
            print(f"✓ Completed builds for hour {hour}")
    
    def execute_builds(self, instance_id, builds):
        """Execute builds on instance"""
        for build in builds:
            flake_url = build['flake_url']
            
            # SSH to instance and build
            cmd = f"ssh ec2-user@{instance_id} 'nix build {flake_url}'"
            subprocess.run(cmd, shell=True)
            
            # Upload to binary cache
            cmd = f"ssh ec2-user@{instance_id} 'nix copy --to s3://nix-cache {flake_url}'"
            subprocess.run(cmd, shell=True)
```

## Economic Model

### Cost Calculation

```python
def calculate_build_cost(schedule):
    """Calculate total cost of scheduled builds"""
    
    total_cost = 0
    
    for slot in schedule:
        # Spot instance cost
        spot_cost = slot['spot_price'] * slot['instances'] * 1  # 1 hour
        
        # Network transfer cost
        transfer_cost = slot['total_gb'] * 0.09  # $0.09/GB
        
        # Storage cost
        storage_cost = slot['total_gb'] * 0.023 / 30  # $0.023/GB/month
        
        slot_cost = spot_cost + transfer_cost + storage_cost
        total_cost += slot_cost
    
    return total_cost

# Example:
# On-demand cost: $100/day
# Spot cost: $30/day (70% savings)
# With prediction markets: $25/day (75% savings)
```

### Arbitrage Strategy

```python
class SpotArbitrage:
    def find_arbitrage_opportunities(self):
        """Find profitable arbitrage opportunities"""
        
        opportunities = []
        
        # Check each region
        for region in REGIONS:
            # Get spot price
            spot_price = self.get_spot_price(region)
            
            # Get market prediction for this region
            prediction = self.get_regional_prediction(region)
            
            # Calculate expected value
            expected_builds = prediction['expected_builds']
            build_value = expected_builds * 0.10  # $0.10 per build
            
            # Calculate cost
            cost = spot_price * 1  # 1 hour
            
            # Profit
            profit = build_value - cost
            
            if profit > 0:
                opportunities.append({
                    'region': region,
                    'spot_price': spot_price,
                    'expected_builds': expected_builds,
                    'profit': profit,
                    'roi': profit / cost
                })
        
        # Sort by ROI
        opportunities.sort(key=lambda x: x['roi'], reverse=True)
        return opportunities
```

## Multi-Cloud Strategy

### Cloud Comparison

```python
CLOUD_PROVIDERS = {
    'aws': {
        'regions': ['us-east-1', 'eu-west-1', 'ap-northeast-1'],
        'instance_type': 'c6i.8xlarge',
        'typical_spot_price': 0.50,
    },
    'gcp': {
        'regions': ['us-central1', 'europe-west1', 'asia-northeast1'],
        'instance_type': 'c2-standard-30',
        'typical_spot_price': 0.45,
    },
    'azure': {
        'regions': ['eastus', 'westeurope', 'japaneast'],
        'instance_type': 'F32s_v2',
        'typical_spot_price': 0.48,
    },
}

def find_cheapest_cloud(region_type='americas'):
    """Find cheapest cloud provider for region"""
    prices = {}
    
    for cloud, config in CLOUD_PROVIDERS.items():
        # Get spot price for this cloud
        price = get_spot_price(cloud, region_type)
        prices[cloud] = price
    
    cheapest = min(prices.items(), key=lambda x: x[1])
    return cheapest
```

## Integration with Markets

### Market-Driven Spot Buying

```python
class MarketDrivenSpotBuyer:
    def buy_based_on_markets(self):
        """Buy spot instances based on market signals"""
        
        # Get predictions
        predictions = market_client.get_predictions('tomorrow')
        
        # Calculate expected demand
        expected_demand = {}
        for pred in predictions:
            region = pred['region']
            hour = pred['hour']
            
            key = (region, hour)
            if key not in expected_demand:
                expected_demand[key] = 0
            
            expected_demand[key] += pred['expected_cpu_hours']
        
        # For each demand peak
        for (region, hour), cpu_hours in expected_demand.items():
            # Get spot price forecast
            spot_price = self.forecast_spot_price(region, hour)
            
            # Calculate ROI
            revenue = cpu_hours * 0.10  # $0.10 per CPU hour
            cost = spot_price * (cpu_hours / 32)  # c6i.8xlarge = 32 vCPUs
            roi = (revenue - cost) / cost
            
            # Only buy if ROI > 20%
            if roi > 0.20:
                instances = int(cpu_hours / 32) + 1
                self.schedule_spot_purchase(region, hour, instances, spot_price)
```

## Monitoring & Optimization

### Real-Time Optimization

```python
class RealTimeOptimizer:
    def optimize_continuously(self):
        """Continuously optimize based on spot prices"""
        
        while True:
            # Check current spot prices
            prices = spot_monitor.get_spot_prices()
            
            # Check running instances
            running = self.get_running_instances()
            
            # For each running instance
            for instance in running:
                current_price = prices[instance.region]
                
                # If price spiked, migrate to cheaper region
                if current_price > instance.max_price * 1.5:
                    # Find cheaper region
                    cheaper_region, cheaper_price = self.find_cheaper_region()
                    
                    # Migrate build
                    self.migrate_build(instance, cheaper_region)
            
            time.sleep(60)  # Check every minute
```

## Vision

**Perfect just-in-time compute:**

```
07:00 → Markets predict Asia builds at 09:00
07:30 → Spot prices drop in Tokyo region
07:45 → Buy 10x c6i.8xlarge @ $0.40/hour
08:00 → Instances ready, pre-download flakes
09:00 → Asia workday starts, builds execute
10:00 → Builds complete, upload to cache
10:15 → Terminate instances
10:30 → Prepare for Europe (12:00 GMT)

Cost: $4 (10 instances × $0.40 × 1 hour)
Value: $32 (320 CPU hours × $0.10)
Profit: $28 (700% ROI)
```

**Result:** Buy compute exactly when and where needed, guided by prediction markets, optimized by spot pricing!
