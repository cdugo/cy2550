BINARY_NAME=xkcdpwgen
DATA_FILE=words.txt

all: 
	release
	
release:
	@echo "Building $(BINARY_NAME)..."
	@cargo build --release
	@echo "$(BINARY_NAME) built successfully."

copy-data:
	@echo "Copying $(DATA_FILE) to target/release..."
	@cp $(DATA_FILE) target/release/
	@echo "$(DATA_FILE) copied successfully."

build: release copy-data

# clean:
# 	@echo "Cleaning up..."
# 	@cargo clean
# 	@echo "Cleaned up."

test:
	@echo "This is a test"
