#include "../common.cpp"
#include <vector>
void    printBoard(char list_current[3][3])
{
    // This function prints the current game
    for (int x = 0; x < 3; x++)
    {
        std::cout << "|" << list_current[x][0] << "|" << list_current[x][1] << "|" << list_current[x][2] << "|\n";
    }
}
bool winTest(char p, char l[3][3])
{
    // This function tests if the player has won
    if ((l[0][0] == p && l[0][1] == p && l[0][2] == p) || (l[0][0] == p && l[1][0] == p && l[2][0] == p) || (l[0][0] == p && l[1][1] == p && l[2][2] == p) || (l[2][0] == p && l[1][1] == p && l[0][2] == p) || (l[2][0] == p && l[2][1] == p && l[2][2] == p) || (l[1][0] == p && l[1][1] == p && l[1][2] == p) || (l[0][1] == p && l[1][1] == p && l[2][1] == p) || (l[0][2] == p && l[1][2] == p && l[2][2] == p))
    {
        return true;
    }
    else
    {
        return false;
    }
}
std::vector<int> removeElement(std::vector<int> vector, int index)
{
    // This function removes an element from a vector
    std::vector<int> new_vector;
    for (int i = 0; i < vector.size(); i++)
    {
        if (i != index)
        {
            new_vector.push_back(vector[i]);
        }
    }
    return new_vector;
}
int main()
{
    char player1 = 'X';
    char player2 = 'O';
    char positions[9][2] = {{0, 0}, {0, 1}, {0, 2}, {1, 0}, {1, 1}, {1, 2}, {2, 0}, {2, 1}, {2, 2}};
    char board[3][3] = {{' ', ' ', ' '},
                        {' ', ' ', ' '},
                        {' ', ' ', ' '}};
    char reference[3][3] = {{'1', '2', '3'},
                            {'4', '5', '6'},
                            {'7', '8', '9'}};
    int play,couny = 0;
    std::cout << "Welcome to Tic Tac Toe!\n";
    std::cout << "Player 1 is X and Player 2 is O\n";
    std::cout << "Player 1 goes first\n";
    std::cout << "To play, enter the number of the position you want to play\n";
    printBoard(reference);
    std::vector<int> playsLeft = {1, 2, 3, 4, 5, 6, 7, 8, 9};
    bool valid;
    do
    {
            do{
            valid = false;
            std::cout << "Player 1, enter your play: ";
            std::cin >> play;
            for (int i = 0; i < playsLeft.size(); i++)
            {
                if (play == playsLeft[i])
                {
                    board[positions[play - 1][0]][positions[play - 1][1]] = player1;
                    printBoard(board);
                    playsLeft = removeElement(playsLeft, play);
                    valid = true;
                    couny++;
                    break;
                }
            }
            } while (valid);
            if (winTest(player1, board))
            {
                std::cout << "Player 1 wins!\n";
                break;
            }
            do
            {
                valid = false;
                std::cout << "Player 2, enter your play: ";
                std::cin >> play;
                for (int i = 0; i < playsLeft.size(); i++)
                {
                    if (play == playsLeft[i])
                    {
                        board[positions[play - 1][0]][positions[play - 1][1]] = player2;
                        printBoard(board);
                        playsLeft = removeElement(playsLeft, play);
                        valid = true;
                        couny++;
                        break;
                    }
                }
            } while (valid);
            
            if (winTest(player2, board))
            {
                std::cout << "Player 2 wins!\n";
                break;
            }
            else if (couny == 9)
            {
                std::cout << "It's a tie!\n";
                break;
            }
    } while (true);
    return 0;
}