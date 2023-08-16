BINARY_NAME=xkcdpwgen
DATA_FILE=words.txt

all: release
	
release: copy-data move-binary-to-parent

copy-data:
	@echo "Ensuring src/target/release/ exists..."
	@mkdir -p src/target/release/
	@echo "Copying $(DATA_FILE) to src/target/release..."
	@cp $(DATA_FILE) src/target/release/
	@echo "$(DATA_FILE) copied successfully."

move-binary-to-parent:
	@echo "Moving binary to parent directory..."
	mv src/target/release/$(BINARY_NAME) $(BINARY_NAME)

build: copy-data move-binary-to-parent

# clean:
# 	@echo "Cleaning up..."
# 	@cargo clean
# 	@echo "Cleaned up."

test:
	@echo "This is a test"

