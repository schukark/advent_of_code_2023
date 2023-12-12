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
            score++;
        }
    }

    return score;
}

int main() {
    std::string line;

    std::vector<int64_t> copies(206, 1);
    int index = 0;

    while (!fin.eof()) {
        std::getline(fin, line);

        int64_t tmp = get_result(line);

        for (int i = index + 1; i <= index + tmp; i++) {
            copies[i] += copies[index];
        }
        index++;
    }

    std::cout << std::accumulate(copies.begin(), copies.begin() + index, 0LL) << std::endl;
}