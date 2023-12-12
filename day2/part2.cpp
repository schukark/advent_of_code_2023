#include <bits/stdc++.h>

std::ifstream fin("input.txt");

struct entry {
    int red = 0, blue = 0, green = 0;
};

entry parse_entry(std::stringstream& iss) {
    char c;
    entry answer;
    bool exit_flag = false;

    while (!exit_flag && !iss.eof()) {
        for (int i = 0; i < 3; i++) {
            if (iss.peek() == ';' || iss.eof()) break;

            int number;
            iss >> number;

            std::string color;
            iss >> color;

            if (color.find("green") != std::string::npos) {
                answer.green = number;
            }
            else if (color.find("blue") != std::string::npos) {
                answer.blue = number;
            }
            else if (color.find("red") != std::string::npos) {
                answer.red = number;
            }

            if (color.back() == ';') {
                exit_flag = true;
                break;
            }
        }
    }

    return answer;
}

std::vector<entry> get_game_info(const std::string& game) {
    int index = 0;
    std::stringstream iss(game);
    char c;
    iss >> c;

    while (c != ':') iss >> c;

    std::vector<entry> answer;

    while (!iss.eof()) {
        answer.push_back(parse_entry(iss));
    }

    return answer;
}

int main() {
    int64_t ans = 0;

    while (!fin.eof()) {
        std::string line;
        std::getline(fin, line);
        auto answer = get_game_info(line);

        int id = 0;
        sscanf(line.c_str(), "Game %d", &id);

        int max_red = -1e9, max_blue = -1e9, max_green = -1e9;

        for (const auto& [red, blue, green] : answer) {
            max_red = std::max(red, max_red);
            max_green = std::max(green, max_green);
            max_blue = std::max(blue, max_blue);
        }

        ans += max_red * max_blue * max_green;
    }

    std::cout << ans << std::endl;
}