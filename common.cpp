#include <iostream>
#include <string>
#include <algorithm>

std::string toLower(std::string str)
{
    std::transform(str.begin(), str.end(), str.begin(), [](unsigned char c)
                   { return std::tolower(c); });
    return str;
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