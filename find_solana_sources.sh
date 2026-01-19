#!/bin/bash
# Find GitHub sources for Solana contracts

CONTRACTS=(
  "Jupiter:JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB"
  "Orca:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"
  "Raydium:675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"
  "Phoenix:PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY"
  "Serum:9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin"
  "Solend:So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo"
  "Mango:mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68"
  "Marinade:MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"
  "Saber:SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ"
  "Drift:dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH"
)

OUTPUT="/mnt/data1/meta-introspector/data/solana_contract_sources.txt"
> "$OUTPUT"

for contract in "${CONTRACTS[@]}"; do
  name="${contract%%:*}"
  address="${contract##*:}"
  
  echo "=== $name ($address) ===" | tee -a "$OUTPUT"
  
  # Search by name
  echo "Searching: $name solana" | tee -a "$OUTPUT"
  
  # Search by address
  echo "Searching: $address" | tee -a "$OUTPUT"
  
  echo "" | tee -a "$OUTPUT"
done

echo "Results saved to: $OUTPUT"
