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

std::vector<std::pair<uint32_t, uint32_t>> seeds;

void create_seeds() {
    std::string input;
    std::getline(fin, input);

    std::stringstream iss(input.substr(7, input.size() - 7));

    while (!iss.eof() && iss.peek() != '\n') {
        uint32_t range_start, range_len;
        iss >> range_start >> range_len;

        seeds.push_back({range_start, range_start + range_len});
    }

    std::getline(fin, input);

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
        tmp.length--;

        transforms.back().push_back(tmp);

        if (fin.eof()) {
            return;
        }

        std::getline(fin, line);
    }
}

void do_transform(uint32_t index) {
    std::vector<std::pair<uint32_t, uint32_t>> ans;
    for (auto& it : seeds) {
        bool flag = false;
        for (const auto& transform : transforms[index]) {
            if ((transform.end + transform.length <= it.first || it.second <= transform.end)) {
                continue;
            }

            if (transform.end <= it.first && it.second <= transform.end + transform.length) {
                ans.push_back({it.first - transform.end + transform.start, it.second - transform.end + transform.start});
                flag = true;
                continue;
            }

            flag = true;
            int left_boundary = std::max(transform.end, it.first);
            int right_boundary = std::min(transform.end + transform.length, it.second);

            ans.push_back({left_boundary - transform.end + transform.start, right_boundary - transform.end + transform.start});
        }

        if (!flag) {
            ans.push_back(it);
        }
    }

    seeds = ans;
    std::cout << "Did transform #" << index << std::endl;
}

int main() {
    create_seeds();

    for (int i = 0; i < 7; i++) {
        input_transform();
    }

    for (int i = 0; i < 7; i++) {
        do_transform(i);

        std::for_each(seeds.begin(), seeds.end(), [](auto x) {
            std::cout << x.first << " " << x.second << std::endl;
        });
        std::cout << std::endl;
    }

    uint32_t ans = std::numeric_limits<uint32_t>::max();

    for (const auto& [first, second] : seeds) {
        ans = std::min(ans, first);
    }

    std::cout << ans << std::endl;
}