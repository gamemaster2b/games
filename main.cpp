#include <iostream>
#include "./common.cpp"
#include "./tic_tac_toe/tic_tac_toe.cpp"
#include "./rock_paper_scissors/rps.cpp"

int main()
{
    std::cout << "This are the games I have made;" << std::endl;
    std::cout << "\t1. Tic Tac Toe" << std::endl;
    std::cout << "\t2. Rock, Paper, Scissors" << std::endl;
    bool check = false;
    std::string yes[] = {"y", "yes", "yeah", "yep", "yup"};
    do
    {
        std::cout << "Please enter the number of the game you want to play: ";
        int game;
        std::cin >> game;
        switch (game)
        {
        case 1:
            ticTacToe();
            break;
        case 2:
            rps();
            break;
        default:
            std::cout << "Please enter a valid number" << std::endl;
            check = true;
            break;
        }
    } while (check);
    std::cout << "Do you want to play another game? (y/n): ";
    std::string play_again;
    std::cin >> play_again;
    if (validReply(toLower(play_again), yes))
    {
        main();
    }
    else
    {
        std::cout << "Thank you for playing" << std::endl;
        return 0;
    }
}