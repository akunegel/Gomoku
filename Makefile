NAME        = gomoku
CARGO       = cargo
CARGO_NAME  = gom
BUILD_DIR   = target/release
BINARY      = $(BUILD_DIR)/$(CARGO_NAME)
SRC         = $(shell find src -name '*.rs')

GREEN       = \033[0;32m
RESET       = \033[0m

all: $(NAME)

$(NAME): $(SRC)
	@echo "Building $(NAME) in release mode..."
	@$(CARGO) build --release
	@cp $(BINARY) $(NAME)
	@echo "$(GREEN)Build successful! Run ./$(NAME) to play.$(RESET)"

clean:
	@echo "Cleaning build files..."
	@$(CARGO) clean

fclean: clean
	@echo "Removing binary..."
	@rm -f $(NAME)

re: fclean all

run:
	@$(CARGO) run --release

test:
	@$(CARGO) test --release

.PHONY: all clean fclean re run test