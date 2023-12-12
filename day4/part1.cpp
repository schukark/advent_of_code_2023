#include <bits/stdc++.h>

std::ifstream fin("input.txt");

int64_t get_result(const std::string& game) {
    std::stringstream iss(game);

    std::string trash;
    char c;

    iss >> trash;
    assert(trash == "Card");

    while (c != ':') iss >> c;

    std::unordered_set<int> winning_words;

    while (iss.peek() != '|') {
        int number;
        iss >> number;
        iss.get();

        winning_words.insert(number);
    }
    iss.get();

    int64_t score = 0;

    while (!iss.eof()) {
        int number;
        iss >> number;
        
        if (winning_words.count(number) > 0) {
            if (score == 0) {
                score = 1;
            }
            else {
                score *= 2;
            }
        }
    }

    return score;
}

int main() {
    std::string line;

    int64_t result = 0;

    while (!fin.eof()) {
        std::getline(fin, line);

        result += get_result(line);
    }

    std::cout << result << std::endl;
}