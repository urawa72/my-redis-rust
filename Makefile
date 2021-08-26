channels:
	mini-redis-server &
	cargo run -p channels
	cargo run -p channels
	pkill -f "mini-redis-server"

.PHONY: channels
