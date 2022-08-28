# Car Calc

A simple Rest API to get Fuel usage for a given distance & probability of fuel injector failure.

## How to start

After cloning the repo

```bash
cargo build
cargo run
```

The Server will run on port 8000. The required endpoints can be found at:

```
http://localhost:8000/api/calculateDieselUsageForDistance
http://localhost:8000/api/probabilityOfUnitInjectorFail
```
