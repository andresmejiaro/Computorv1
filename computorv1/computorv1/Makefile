# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: amejia <amejia@student.42.fr>              +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2024/02/10 18:04:55 by amejia            #+#    #+#              #
#    Updated: 2024/02/11 13:49:28 by amejia           ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME = computorv1
SRCS = main.rs src parsing.rs polinomial.rs

all: $(NAME)
	@RUSTFLAGS="-D warnings" cargo build --release
	@cp target/release/$(NAME) .
	
debug: 
	cargo build 
	@cp target/debug/$(NAME) .


$(NAME): src/$(SRCS)
	
	

src/$(SRCS):
	

clean:
	@cargo clean

fclean: clean
	@rm -f $(NAME)

re: fclean all

run: all
	@./$(NAME)

.PHONY: all clean fclean re run
