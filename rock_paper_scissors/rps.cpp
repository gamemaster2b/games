#include <iostream>
#include <string>
#include <algorithm>
#include <cctype>
#include <ctime>

std::string toLower(std::string str)
{
	std::transform(str.begin(), str.end(), str.begin(), [](unsigned char c)
				   { return std::tolower(c); });
	return str;
}
bool win_test(std::string player1, std::string player2)
{
	return (((player1 == "rock" || player1 == "r") && (player2 == "scissors" || player2 == "s")) ||
			((player1 == "scissors" || player1 == "s") && (player2 == "paper" || player2 == "p")) ||
			((player1 == "paper" || player1 == "p") && (player2 == "rock" || player2 == "r")));
}
bool validReply(std::string play, std::string choices[])
{
	bool valid = false;
	for (int i = 0; i < 6; i++)
	{
		if (play == choices[i])
		{
			valid = true;
			break;
		}
	}
	return valid;
}
int main()
{
	std::cout << "Welcome to Rock🪨, Paper🧻, Scissors✂️\n";
	int rounds, count;
	std::string play, choices[] = {"rock", "paper", "scissors", "r", "p", "s"}, yes[] = {"y", "yes", "yeah", "yep", "yup"};
	do
	{
		do
		{
			std::cout << "Please enter a positive integer for the number of rounds: ";
			std::cin >> rounds;
		} while (rounds <= 0);
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
			srand(time(NULL));
			int random = rand() % 3;
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
		std::cin >> play;
	} while (validReply(toLower(play), yes));
	return 0;
}