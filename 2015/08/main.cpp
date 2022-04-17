#include <fstream>
#include <array>
#include <regex>
#include <iostream>
#include <algorithm>
#include <stdint.h>

std::regex sep{"[ ]+"};

using std::cout;
int main() {
    int total = 0;
    int filtered = 0;
    int bigger = 0;
    std::regex filter{"\\\\\"|\\\\\\\\|\\\\[xX][a-fA-F0-9]{2}"};
    std::regex quotes{"\""};
    std::regex scape{"\\\\\""};
    std::regex hex{"\\\\[xX][a-fA-F0-9]{2}"};
    std::regex quotes_start{"^\""};
    std::regex quotes_end{"\"\r|\"$"};
    std::string line;
    std::ifstream data("data.json");
    std::vector<std::string> inputs;

    if (data.is_open())
    {
        while(getline(data, line))
        {
            total += line.length();
            auto arguments = std::regex_replace(std::regex_replace(line, filter, "#"), quotes, "");

            auto encoded = std::regex_replace(line, scape, "\\\\\\\"");
            encoded = std::regex_replace(encoded, hex, "\\\\x##");
            encoded = std::regex_replace(encoded, quotes_start, "\"\\\"");
            encoded = std::regex_replace(encoded, quotes_end, "\\\"\"");
            filtered += arguments.length();
            bigger += encoded.length();
            cout<< encoded<<std::endl;
        }
        data.close();
    }
    cout << total << std::endl;
    cout << filtered << std::endl;
    cout << total - filtered << std::endl;
    cout << bigger << std::endl;
    cout << bigger - total << std::endl;

    return 0;
}
