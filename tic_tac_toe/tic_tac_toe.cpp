#include <iostream>
#include <ctime>
#include <vector>
#include <cstdlib>
#include <string>

void current_game(char list_current[3][3])
{
    // This function prints the current game
    for (int x = 0; x < 3; x++)
    {
        std::cout << " _ _ _\n";
        std::cout << "|" << list_current[x][0] << "|" << list_current[x][1] << "|" << list_current[x][2] << "|\n";
    }
    std::cout << " _ _ _\n";
}

bool win_test(char p, char l[3][3])
{
    // This function tests if the player has won
    if ((l[0][0] == p && l[0][1] == p && l[0][2]) || (l[0][0] == p && l[1][0] == p && l[2][0]) || (l[0][0] == p && l[1][1] == p && l[2][2] == p) || (l[2][0] == p && l[1][1] == p && l[0][2] == p) || (l[2][0] == p && l[2][1] == p && l[2][2] == p) || (l[1][0] == p && l[1][1] == p && l[1][2] == p) || (l[0][1] == p && l[1][1] == p && l[2][1] == p) || (l[0][2] == p && l[1][2] == p && l[2][2] == p))
    {
        return true;
    }
    else
    {
        return false;
    }
}

int main()
{
    // Here I am declaring the variables
    char A = ' ';
    char B = 'X';
    char C = 'O';
    char play;
    char winner;
    int plays_available = 9;
    int computer_play;
    char computer_play_char;

    // Here I set the default values for the game
    std::vector<char> positions = {'1', '2', '3', '4', '5', '6', '7', '8', '9'};
    char default_value[3][3] = {{'1', '2', '3'}, {'4', '5', '6'}, {'7', '8', '9'}};
    char position_value[3][3] = {{A, A, A}, {A, A, A}, {A, A, A}};

    // Here I am printing the welcome message and the board positions
    std::cout << "Welcome to tic tac toe.\nThe following are the board positons:\n";
    current_game(default_value);
    bool game_over = false, valid_play = false, round_over = false;
    while (game_over == false)
    {

        while (round_over == false)
        {
            std::cout << "This is the cureent game:\n";
            current_game(position_value);
            while (valid_play == false)
            {
                std::cout << "Make your play:";
                std::cin >> play;

                for (int i = 0; i < 3; i++)
                {
                    for (int j = 0; j < 3; j++)
                    {
                        if (play == default_value[i][j])
                        {
                            if (position_value[i][j] == A)
                            {
                                position_value[i][j] = B;
                                valid_play = true;
                                current_game(position_value);
                            }
                            else
                            {
                                std::cout << "Invalid play. Please choose a valid position.";
                            }
                        }
                    }
                }
            }
            valid_play = false;
            plays_available--;
            std::cout << "Plays available: " << plays_available << std::endl;

            // Here I am making the computer play
            while (valid_play == false && plays_available > 0)
            {
                srand(time(0));
                computer_play = 1 + (rand() % 9);
                switch (computer_play)
                {
                case 0:
                    computer_play_char = '7';
                    break;
                case 1:
                    computer_play_char = '1';
                    break;
                case 2:
                    computer_play_char = '2';
                    break;
                case 3:
                    computer_play_char = '3';
                    break;
                case 4:
                    computer_play_char = '4';
                    break;
                case 5:
                    computer_play_char = '5';
                    break;
                case 6:
                    computer_play_char = '6';
                    break;
                case 7:
                    computer_play_char = '7';
                    break;
                case 8:
                    computer_play_char = '8';
                    break;
                case 9:
                    computer_play_char = '9';
                    break;
                default:
                    break;
                }
                
                // Here I am testing if the computer play is valid
                for (int i = 0; i < 3; i++)
                {
                    for (int j = 0; j < 3; j++)
                    {
                        if (computer_play_char == default_value[i][j])
                        {
                            if (position_value[i][j] == A)
                            {
                                position_value[i][j] = C;
                                valid_play = true;
                                current_game(position_value);
                            }
                        }
                    }
                }
            }
            valid_play = false;
            plays_available--;
            std::cout << "Plays available: " << plays_available << std::endl;

            // Here I am testing if the game is over
            if (win_test(B, position_value) == true)
            {
                winner = 'p';
                round_over = true;
            }
            else if (win_test(C, position_value) == true)
            {
                winner = 'c';
                round_over = true;
            }
            else if (plays_available <= 0)
            {
                round_over = true;
            }
        }
        if (winner == 'p')
        {
            std::cout << "🎉You won!🎉"<<std::endl;
        }
        else if (winner == 'c')
        {
            std::cout << "You lost!😢"<<std::endl;
        }
        else
        {
            std::cout << "It's a tie!😑"<<std::endl;
        }

        // Here I am asking the player if they want to play again
        std::cout << "Do you want to play again? (y/n):";
        std::string play_again;
        std::cin >> play_again;
        if (play_again == "y" || play_again == "Y" || play_again == "yes" || play_again == "Yes" || play_again == "YES" || play_again == "yEs" || play_again == "yeS" || play_again == "YEs" || play_again == "yES" || play_again == "YeS" || play_again == "yEs" || play_again == "yES" || play_again == "YeS")
        {
            game_over = false;
            round_over = false;
            plays_available = 9;
            winner = NULL;
            for (int i = 0; i < 3; i++)
            {
                for (int j = 0; j < 3; j++)
                {
                    position_value[i][j] = A;
                }
            }
        }
        else
        {
            game_over = true;
        }
    }

    std::cout << "Thanks for playing!";
    return 0;
}