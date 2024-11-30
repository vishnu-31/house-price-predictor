run-train-dev:
	cargo run --bin train -- \
		-s house-price-prediction-project -k xgboost_model.bin

run-api-dev:
	cargo run --bin api -- \
		-s house-price-prediction-project -k xgboost_model.bin

run-release:
	cargo run --release
	./target/release/house-price-predictor

request-health-dev:
	curl http://localhost:8080/health

request-predict-dev:
	curl -X POST http://localhost:8080/predict \
		-H "Content-Type: application/json" \
		-d '{"crim": 0.00632, "zn": 18.0, "indus": 2.31, "chas": 0, "nox": 0.538, "rm": 6.575, "age": 65.2, "dis": 4.09, "rad": 1, "tax": 296, "ptratio": 15.3, "b": 396.9, "lstat": 4.98}'