#pragma GCC optimize("O2")
#pragma GCC target("avx2")

#include <bits/stdc++.h>
#include <cassert>

std::ifstream fin("input.txt");

struct interval {
    uint32_t start, end, length;

    interval(uint32_t start = 0, uint32_t end = 0, uint32_t length = 0): start(start), end(end), length(length) {}

    bool operator<(const interval& other) const {
        if (start == other.start) {
            return end < other.end;
        }
        return start < other.start;
    }
};

std::vector<uint32_t> seeds;
std::vector<bool> marker(std::numeric_limits<uint32_t>::max(), false);

void create_seeds() {
    std::string input;
    std::getline(fin, input);

    std::stringstream iss(input.substr(7, input.size() - 7));

    while (!iss.eof() && iss.peek() != '\n') {
        uint32_t range_start, range_len;
        iss >> range_start >> range_len;

        for (uint32_t i = 0; i < range_len; i++) {
            marker[range_start + i] = true;
        }
        std::cout << "Inputted range with length = " << range_len << std::endl; 
    }

    std::getline(fin, input);

    for (int i = 0; i < marker.size(); i++) {
        if (marker[i]) {
            seeds.push_back(i);
        }
    }

    std::cout << "Made seeds" << std::endl;
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

void do_transform(uint32_t index) {
    for (auto& it : seeds) {
        for (const auto& transform : transforms[index]) {
            if (!(transform.end <= it && it <= transform.end + transform.length)) {
                continue;
            }

            it = it - transform.end + transform.start;
            break;
        }
    }

    std::cout << "Did transform #" << index << std::endl;
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