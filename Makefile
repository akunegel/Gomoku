NAME			= Gomoku
CARGO			= cargo
BUILD_DIR		= target/release
BINARY			= $(BUILD_DIR)/gomoku_screen

all: $(NAME)

$(NAME):
		@$(CARGO) build --release
		@cp $(BINARY) $(NAME)

clean:
		@$(CARGO) clean

fclean: clean
		@rm -f $(NAME)

re: fclean all

.PHONY: all clean fclean re