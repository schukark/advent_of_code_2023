#include <bits/stdc++.h>

std::ifstream fin("input.txt");

std::vector<std::string> numbers = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

int parse_string(std::string current) {
    int tmp = 0;
    bool flag = false;
    for (int i = 0; i < current.size(); i++) {
        if (flag) {
            break;
        }

        if ('0' <= current[i] && current[i] <= '9') {
            tmp = current[i] - '0';
            break;
        }

        for (int j = 0; j < numbers.size(); j++) {
            std::string word = numbers[j];
            if (current.substr(i, word.size()) == word) {
                tmp = j + 1;
                flag = true;
                break;
            }
        }
    }

    flag = false;

    for (int i = current.size() - 1; i >= 0; i--) {
        if (flag) {
            break;
        }
        if ('0' <= current[i] && current[i] <= '9') {
            tmp = tmp * 10 + (current[i] - '0');
            break;
        }

        for (int j = 0; j < numbers.size(); j++) {
            std::string word = numbers[j];
            if (current.substr(i, word.size()) == word) {
                tmp = tmp * 10 + (j + 1);
                flag = true;
                break;
            }
        }
    }

    return tmp;
}

int main() {
    std::string current;
    int64_t ans = 0;

    while (!fin.eof()) {
        fin >> current;

        int tmp = parse_string(current);

        ans += tmp;
    }

    std::cout << ans << std::endl;
}