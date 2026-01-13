#!/bin/bash

# Thermal Work Measurement System
# Measures computational work done via CPU temperature delta

BASELINE_TEMP=22
LOG_FILE="/mnt/data1/meta-introspector/analysis/thermal_work.log"

echo "🌡️  THERMAL WORK MEASUREMENT SYSTEM" | tee -a "$LOG_FILE"
echo "===================================" | tee -a "$LOG_FILE"
echo "$(date): Starting thermal monitoring" | tee -a "$LOG_FILE"

while true; do
    # Get current max CPU temp
    CURRENT_TEMP=$(sensors | grep "Core" | awk '{print $3}' | sed 's/+//g' | sed 's/°C//g' | sort -n | tail -1)
    
    if [ ! -z "$CURRENT_TEMP" ]; then
        # Calculate work delta
        WORK_DELTA=$(echo "$CURRENT_TEMP - $BASELINE_TEMP" | bc -l)
        
        # Log thermal work measurement
        echo "$(date): CPU=${CURRENT_TEMP}°C, Work Delta=+${WORK_DELTA}°C" | tee -a "$LOG_FILE"
        
        # Check if significant work is being done
        if (( $(echo "$WORK_DELTA > 3" | bc -l) )); then
            echo "🔥 High computational load detected: +${WORK_DELTA}°C" | tee -a "$LOG_FILE"
        fi
    fi
    
    sleep 10
done
