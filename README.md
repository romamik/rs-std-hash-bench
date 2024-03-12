This is a very quickhand test of u128 hashes performance.

My main concern was performance of default u128 hash on WASM in browser.

On my machine test results are:
```
cargo run --release

u128 default hash time: 0.24303293228149414
u128 xor hash manual impl time: 0.06629395484924316
sum indices time: 0.002939939498901367

# WASM, Chrome
# run ./build-web.sh and then open index.html using local webserver
u128 default hash time: 4.421899999856949
u128 xor hash manual impl time: 0.30189999985694804
sum indices time: 0.0037000000476830053

# WASM, Firefox
u128 default hash time: 4.881 
u128 xor hash manual impl time: 1.8569999999999993
sum indices time: 0.02400000000000091
```

So, for native target xor hash is ~4 times faster than default hash.

In Chrome default hash is ~20 times (!) slower than native, while xor hash is only ~5 times slower than native. 
Also, in chrome xor hash is ~15 times (!) faster than default hash.

In Firefox default hash performs roughly the same as in firefox, while xor hash is ~30 times slower than native and ~6 times slower than in Chrome.

I've added "sum indices" test to check general performance not bound to hashes. 
In chrome this test runs at basically the same speed as native and in Firefox it is ~10 times slower.
