#include <iostream>
#include <ctime>
#include <vector>
#include <cstdlib>

void current_game(char list_current[3][3]){
    for(int x=0;x<3;x++){
        std::cout<<" _ _ _\n";
        std::cout<<"|"<<list_current[x][0]<<"|"<<list_current[x][1]<<"|"<<list_current[x][2]<<"|\n";
    }
    std::cout<<" _ _ _\n";
}
bool win_test(char p,char l[3][3]){
    if((l[0][0]==p && l[0][1]==p && l[0][2]) || (l[0][0]==p && l[1][0]==p && l[2][0]) || (l[0][0]==p && l[1][1]==p && l[2][2]==p) || (l[2][0]==p && l[1][1]==p && l[0][2]==p) || (l[2][0]==p && l[2][1]==p && l[2][2]==p) || (l[1][0]==p && l[1][1]==p && l[1][2]==p) || (l[0][1]==p && l[1][1]==p && l[2][1]==p) || (l[0][2]==p && l[1][2]==p && l[2][2]==p)){
        return true;
    }
    else{
        return false;
    }
}
void update_positons(char pos[],char play){
    char*p=pos;
    char cur_pos;
}

void utext(){
    char A=' ';
    char B='X';
    char play;
    char C='O';
    int plays_available=9;
    std::vector<char> positions={'1','2','3','4','5','6','7','8','9'};
    char default_value[3][3]={{'1','2','3'},{'4','5','6'},{'7','8','9'}};
    char position_value[3][3]={{A,A,A},{A,A,A},{A,A,A}};
    std::cout<<"Welcome to tic tac toe.\nThe following are the board positons:\n";
    current_game(default_value);
    bool game_over=false,player_won=false,comp_won=false;
    while(game_over==false){
        std::cout<<"This is the cureent game:\n";
        current_game(position_value);
        std::cout<<"Make your play:";
        std::cin>>play;plays_available--;
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