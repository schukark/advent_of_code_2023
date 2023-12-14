#include <bits/stdc++.h>
#include <cassert>

std::ifstream fin("input.txt");

struct interval {
    int64_t start, end, length;

    interval(int64_t start = 0, int64_t end = 0, int64_t length = 0): start(start), end(end), length(length) {}

    bool operator<(const interval& other) const {
        if (start == other.start) {
            return end < other.end;
        }
        return start < other.start;
    }
};

std::vector<int64_t> seeds;

void create_seeds() {
    std::string input;
    std::getline(fin, input);

    std::stringstream iss(input.substr(7, input.size() - 7));

    while (!iss.eof() && iss.peek() != '\n') {
        int64_t number;
        iss >> number;

        seeds.push_back(number);
    }

    std::getline(fin, input);
}

std::vector<std::vector<interval>> transforms;

void input_transform() {
    transforms.push_back({});

    std::string line;
    std::getline(fin, line);
    std::getline(fin, line);

    while (line != "\n" && line != "") {
        std::stringstream iss(line);
        interval tmp;

        iss >> tmp.start;
        iss >> tmp.end;
        iss >> tmp.length;

        transforms.back().push_back(tmp);

        if (fin.eof()) {
            return;
        }

        std::getline(fin, line);
    }
}

void do_transform(int64_t index) {
    for (auto& it : seeds) {
        for (const auto& transform : transforms[index]) {
            if (!(transform.end <= it && it <= transform.end + transform.length)) {
                continue;
            }

            it = it - transform.end + transform.start;
            break;
        }
    }
}

int main() {
    create_seeds();

    for (int i = 0; i < 7; i++) {
        input_transform();
    }

    for (int i = 0; i < 7; i++) {
        do_transform(i);
    }

    std::cout << *std::min_element(seeds.begin(), seeds.end());
}