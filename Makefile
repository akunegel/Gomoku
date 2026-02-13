NAME        = gomoku
CARGO       = cargo
CARGO_NAME  = gom
BUILD_DIR   = target/release
BINARY      = $(BUILD_DIR)/$(CARGO_NAME)

GREEN       = \033[0;32m
RESET       = \033[0m

all: $(NAME)

$(NAME):
	@echo "Building $(NAME) in release mode..."
	@$(CARGO) build --release
	@cp $(BINARY) $(NAME)
	@echo "$(GREEN)Build successful! Run ./$(NAME) to play.$(RESET)"

clean:
	@echo "Cleaning object files..."
	@$(CARGO) clean

fclean: clean
	@echo "Removing binary..."
	@rm -f $(NAME)

re: fclean all

run:
	@$(CARGO) run --release

test:
	@$(CARGO) test

.PHONY: all clean fclean re run test