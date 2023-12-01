#include <algorithm>
#include <cctype>
#include <execution>
#include <iostream>
// #include <experimental/ranges/algorithm>
#include <fstream>
#include <numeric>
#include <range/v3/view/getlines.hpp>
#include <string>
#include <vector>

std::vector<std::string> literals = {"one", "two",   "three", "four", "five",
                                     "six", "seven", "eight", "nine"};

template <typename S>
int get_number(const S& s) {
  // auto ten = std::experimental::ranges::find_if(
  auto ten =
      std::find_if(s.begin(), s.end(), [](auto c) { return std::isdigit(c); });
  auto one = std::find_if(
      s.rbegin(), s.rend(), [](auto c) { return std::isdigit(c); });
  return ((*ten) - '0') * 10 + ((*one) - '0');
}

int main(int argc, char** argv) {
  std::ifstream instream(argv[1]);
  std::vector<std::string> input;
  std::ranges::for_each(
      ranges::getlines_view(instream),
      [&input](const auto line) { input.push_back(line); });

  int result = std::transform_reduce(
      std::execution::par_unseq, input.begin(), input.end(),
      static_cast<int>(0), std::plus<>(),
      [](const auto& line) { return get_number(line); });
  std::cout << "part 1: " << result << '\n';
  return 0;
}
