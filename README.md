# intent-services

A proof-of-concept intent/quote service inspired by [CoW Protocol](https://github.com/cowprotocol/services).

## Running

```bash
RPC_URL=https://mainnet.infura.io/v3/YOUR_KEY cargo run
```

## API

### `POST /quote`

Returns a signed intent quote for a token swap via Uniswap V3.

**Example — sell 1 WETH for USDC:**

```bash
curl -X POST http://localhost:8080/quote \
  -H "Content-Type: application/json" \
  -d '{
    "owner": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
    "receiver": null,
    "sell_token": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
    "buy_token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    "sell_amount": "1000000000000000000",
    "buy_amount": null,
    "slippage_bps": 50,
    "valid_for_sec": 10,
    "app_data": null,
    "price_quality": "Fast"
  }'
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "expires_at": 10,
  "verified": true,
  "intent_to_sign": {
    "order_type": "Sell",
    "owner": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
    "receiver": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
    "sell_token": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
    "buy_token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    "sell_amount": "1000000000000000000",
    "buy_amount": "3241500000"
  }
}
```

**Notes:**
- `sell_amount` / `buy_amount` are in token base units (wei for ETH, 6 decimals for USDC)
- Exactly one of `sell_amount` or `buy_amount` must be set
- `price_quality`: `"Fast"` requires `valid_for_sec >= 10`, `"Optimal"` requires `>= 30`
- Price is fetched from Uniswap V3 QuoterV2 at fee tier 0.3%

## Running tests

```bash
RPC_URL=https://mainnet.infura.io/v3/YOUR_KEY cargo test
```
