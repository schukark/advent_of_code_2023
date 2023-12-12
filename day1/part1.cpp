#include <bits/stdc++.h>

std::ifstream fin("input.txt");

int main() {
    std::string current;
    int64_t ans = 0;

    while (!fin.eof()) {
        fin >> current;

        int tmp = 0;
        for (int i = 0; i < current.size(); i++) {
            if ('0' <= current[i] && current[i] <= '9') {
                tmp = current[i] - '0';
                break;
            }
        }

        for (int i = current.size() - 1; i >= 0; i--) {
            if ('0' <= current[i] && current[i] <= '9') {
                tmp = tmp * 10 + (current[i] - '0');
                break;
            }
        }

        ans += tmp;
    }

    std::cout << ans << std::endl;
}