# keyserver-rust

```sh
RUST_SERVER_URL="keys.walletconnect.com"
RUST_SERVER_URL="159.65.123.131"

curl -i -X DELETE "$RUST_SERVER_URL/keys"

curl -i -X GET "$RUST_SERVER_URL/keys"

curl -i -X POST -H "Content-Type: application/json" \
    -d '{"account": "eip:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826", "publicKey": "2d573da1d2b8dbe3dcdb6ce7de47ce44b18fb8ec5ddc9d3f412ab4a718fff93c"}' \
    "$RUST_SERVER_URL/register"
    
curl -i -X GET "$RUST_SERVER_URL/resolve?account=eip:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
```
