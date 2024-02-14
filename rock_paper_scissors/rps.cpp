#include <iostream>
#include <string>

bool win_test(std::string player1, std::string player2) {
	if ((player1 == "rock" || player1 =="r") && (player2 == "scissors" || player2 == "s")) {
		return true;
	}
	else if ((player1 == "scissors" || player1 == "s") && (player2 == "paper" || player2 == "p")) {
		return true;
	}
	else if ((player1 == "paper" || player1 == "p") && (player2 == "rock" || player2 == "r")) {
		return true;
	}
	else {
		return false;
	}
}

int main() {
	std::string player1, player2, game_mode;
	int player1_score = 0, player2_score = 0;
	std::cout << "Welcome to Rock, Paper, Scissors!\n";
	std::cout << "The are two game modes; pvp(p) and player vs computer(c).\n";
	
	
	return 0;
}