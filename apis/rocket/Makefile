nightly:
	rustup override set nightly

stable:
	rustup override set stable

lint:
	cargo fmt

serve:
	cargo watch -x run

migration:
	diesel migration run --database-url postgresql://rs_user:rs_pass@localhost:5432/postgres
