#include <iostream>
#include "./tic_tac_toe/tic_tac_toe.cpp"
/// #include "./rock_paper_scissors/rps.cpp" // This is not complete yet

int main()
{
    std::cout << "This are the games I have made;" << std::endl;
    std::cout << "\t1. Tic Tac Toe" << std::endl;
    std::cout << "\t2. Rock, Paper, Scissors" << std::endl;
    std::cout << "What would you like to play today: ";
    char game;
    std::cin >> game;
    switch (game)
    {
    case '1':
        ticTacToe();
        break;
    }
}