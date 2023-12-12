#include <bits/stdc++.h>

std::ifstream fin("input.txt");

int n = 0, m = 0;
std::vector<std::pair<int, int>> mat = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}, {1, 1}, {1, -1}, {-1, 1}, {-1, -1}};

std::unordered_map<int, std::unordered_multiset<int64_t>> gears;

void check_boundary(std::vector<std::string>&graph, int row, int column, int number_len, int64_t number) {
    std::unordered_set<int> already_accounted;
    
    for (int i = 0; i < number_len; i++) {
        for (const auto& [dx, dy] : mat) {
            int new_x = row + dx;
            int new_y = column + dy + i;

            if (new_x < 0 || new_y < 0 || new_x >= n || new_y >= m) {
                continue;
            }

            if (graph[new_x][new_y] == '*' && !(graph[new_x][new_y] >= '0' && graph[new_x][new_y] <= '9')) {
                if (already_accounted.count(new_x * m + new_y) > 0) {
                    continue;
                }

                gears[new_x * m + new_y].insert(number);
                already_accounted.insert(new_x * m + new_y);
            }
        }
    }
}

int main() {
    std::string line;
    std::vector<std::string> graph;

    while (!fin.eof()) {
        std::getline(fin, line);

        graph.push_back(line);
        n++;
        m = line.size();
    }

    int start_col = -1, start_row = -1, len = 0;

    int64_t ans = 0;

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            if (graph[i][j] >= '0' && graph[i][j] <= '9') {
                if (start_col == -1) {
                    start_row = i;
                    start_col = j;
                }
                len++;
            }
            else {
                if (start_col != -1) {
                    int64_t tmp = std::stoll(graph[start_row].substr(start_col, len));
                    check_boundary(graph, start_row, start_col, len, tmp);
                    start_row = -1;
                    start_col = -1;
                    len = 0;
                }
            }
        }

        if (start_col != -1) {
            int64_t tmp = std::stoll(graph[start_row].substr(start_col, len));
            check_boundary(graph, start_row, start_col, len, tmp);
            start_row = -1;
            start_col = -1;
            len = 0;
        }
    }

    for (const auto &[key, value] : gears) {
        if (value.size() == 2) {
            auto tmp = value.begin();
            tmp++;

            ans += *(value.begin()) * (*tmp);
        }
    }

    std::cout << ans << std::endl;
}