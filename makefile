build:
	@cargo build --release

publish: build
	@source ./.env && \
	if [ -z "$BLAIR_UPLOAD_TARGET" ]; then \
		echo "Error: BLAIR_UPLOAD_TARGET not set in .env"; \
		exit 1; \
	fi
	@source ./.env && rsync -avz --delete ./target/release/blair $$BLAIR_UPLOAD_TARGET
	@source ./.env && rsync -avz --delete ./static $$WEBSITE_UPLOAD_TARGET
	@source ./.env && ssh $$BLAIR_REMOTE_HOST "$$BLAIR_RESTART_COMMAND"

clean:
	@rm -rf ./target

live:
	@cargo run
