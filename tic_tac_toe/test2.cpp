#include <iostream>
#include <ctime>
#include <vector>
#include <cstdlib>
#include "test.cpp"
int main(){
    char A=' ';
    char B='X';
    char play;
    char C='O';
    int plays_available=9;
    std::vector<char> positions={'1','2','3','4','5','6','7','8','9'};
    char default_value[3][3]={{'1','2','3'},{'4','5','6'},{'7','8','9'}};
    char position_value[3][3]={{A,A,A},{A,A,A},{A,A,A}};
    bool game_over=false,player_won=false,comp_won=false;
        bool valid_play==false;
    while(game_over==false){
        do{
            std::cout<<"This is the cureent game:\n";
        current_game(position_value);
        std::cout<<"Make your play:";
        std::cin>>play;
        valid_play=false;
        for(int i=0;i<plays_available.size();i++){
            if(play==plays_available[i]){
                valid_play=true;
            }
        }
        if(valid_play==true){
            char p_val=B;
            switch(play){
                case '1':
                position_value[0][0]=p_val;
                break;
                case '2':
                position_value[0][1]=p_val;
                break;
                case '3':
                position_value[0][2]=p_val;
                break;
                case '4':
                position_value[1][0]=p_val;
                break;
                case '5':
                position_value[1][1]=p_val;
                break;
                case '6':
                position_value[1][2]=p_val;
                break;
                case '7':
                position_value[2][0]=p_val;
                break;
                case '8':
                position_value[2][1]=p_val;
                break;
                case '9':
                position_value[2][2]=p_val;
                break;
            }
            plays_available=update_positions(plays_available);
            position_value=update_position_value(position_value);
        }
        else{
            std::cout<<"Improper play. Make an appropriate.\n";
        }
        }while(valid_play==false)
        plays_available--;
        if(win_test(B,position_value)==true){
            game_over=true;
        }
        else if(win_test(A,position_value)==true){
            game_over=true;
        }
        else if(plays_available==0){
            game_over=true;
        }       
    }
}