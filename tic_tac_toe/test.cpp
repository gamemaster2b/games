 #include <iostream>
#include <ctime>
#include <vector>
#include <cstdlib>
std::vector<char> update_positions(std::vector<char> cha, char p){
    std::vector<char> newp;
    std::vector<char>*ch=&cha;
    for(int i=0;i<cha.size();i++){
        if(cha[i]!=p){
            std::cout<<cha[i]<<"\t";
            char r=cha[i];
            std::cout<<r<<"\t";
            newp.push_back(r);
            }
        std::cout<<newp[i]<<"\n";
    }
    
    return newp;
}
char update_position_value(char cha[3][3], char p){
    std::vector<char> newp;
    for(int i=0;i<cha.size();i++){
        if(cha[i]!=p){
            std::cout<<cha[i]<<"\t";
            char r=cha[i];
            std::cout<<r<<"\t";
            newp.push_back(r);
            }
        std::cout<<newp[i]<<"\n";
    }
    
    return newp;
}
int main(){
    char A=' ';
    char B='X';
    char play;
    char C='O';
    std::vector<char> positions={'1','2','3','4','5','6','7','8','9'};
    char default_value[3][3]={{'1','2','3'},{'4','5','6'},{'7','8','9'}};
    char position_value[3][3]={{A,A,A},{A,A,A},{A,A,A}};
    play='4';
    positions=update_positions(positions,play);
    std::cout<<std::endl;
    
    for(int i=0;i<positions.size();i++){
        std::cout<<positions[i]<<"\n";
    }
    
    
    
}