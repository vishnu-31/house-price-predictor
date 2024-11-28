run-train-dev:
	cargo run --bin train

run-api-dev:
	cargo run --bin api

run-release:
	cargo run --release
	./target/release/house-price-predictor

request-health-dev:
	curl http://localhost:8080/health