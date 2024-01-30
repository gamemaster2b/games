#include <iostream>
#include <string>
#include <ctime>
#include "../common.cpp"

bool win_test(std::string player1, std::string player2)
{
	return (((player1 == "rock" || player1 == "r") && (player2 == "scissors" || player2 == "s")) ||
			((player1 == "scissors" || player1 == "s") && (player2 == "paper" || player2 == "p")) ||
			((player1 == "paper" || player1 == "p") && (player2 == "rock" || player2 == "r")));
}

int rps()
{
	std::cout << "Welcome to Rock🪨, Paper🧻, Scissors✂️\n";
	int rounds = 0, count = 0;
	srand(time(NULL));
	std::string play, choices[] = {"rock", "paper", "scissors", "r", "p", "s"};
	do
	{
		do
		{
			std::cout << "Please enter a positive integer for the number of rounds: ";
			std::cin >> rounds;
		} while (!rounds > 0);
		std::cout << "\n\n";
		std::cout << "There are three choices;\n(r) rock\n(p) paper\n(s) scissors\n\n";
		do
		{
			count++;
			std::cout << "Round " << count << " !!GO!!" << std::endl;
			do
			{
				std::cout << "What is your choice: ";
				std::cin >> play;
			} while (!validReply(toLower(play), choices));

			int random = rand() % 6;
			std::string computer = choices[random];
			std::cout << "Computer chose " << computer << std::endl;
			if (toLower(play) == computer)
			{
				std::cout << "It's a tie!\n";
			}
			else if (win_test(toLower(play), computer))
			{
				std::cout << "You win!\n";
			}
			else
			{
				std::cout << "You lose!\n";
			}
			rounds--;
			std::cout << "\n\n";
		} while (rounds > 0);
		std::cout << "Would you like to play again? (y/n): ";
		count = 0;
		std::cin >> play;
	} while (validReply(toLower(play), yes));
	return 0;
}