# Solana RPC WASM Plugin

✅ **Built successfully!**

## Output
`pkg/solana_rpc_wasm.js` - WASM module ready to use

## Functions

```javascript
import init, { 
  fetch_block, 
  fetch_signatures, 
  submit_to_server,
  get_contract_address 
} from './pkg/solana_rpc_wasm.js';

await init();

// Get contract address
const contract = get_contract_address();
// "BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump"

// Fetch block
const block = await fetch_block(12345);

// Fetch signatures
const sigs = await fetch_signatures(contract);

// Submit to server
await submit_to_server(JSON.stringify(block), "http://localhost:9000/submit");
```

## Usage in solfunmeme-dioxus

Copy to dioxus project:
```bash
cp -r pkg /mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus/plugins/solana-rpc
```

## Next Steps

1. ✅ WASM plugin built
2. [ ] Create server .so plugin
3. [ ] Wire into solfunmeme-dioxus
4. [ ] Test end-to-end
