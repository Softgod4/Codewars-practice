#include <string>
#include <vector>
#include <iostream>

std::string fakeBin(std::string str){
  std::vector<int> vec;
  for (int i = 0; i < str.length(); i++) {
    if (str[i] < '5') {
      vec.push_back(0);
    } else {
      vec.push_back(1);
    }
  }
  std::string result;
  for (int digit : vec) {
    result += std::to_string(digit);
  }
  return result;
}

int main() {
  std::cout << fakeBin("45385593107843568") << std::endl;
}
