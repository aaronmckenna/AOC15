#include <string>
#include <vector>
#include <iostream>
#include <fstream>
using namespace std;

int main() {
	ifstream readfile;
	vector<char> instructions;
	readfile.open("INPUT");
	char c;
	if (readfile.is_open()) {
		while (!readfile.eof()) {
			readfile.get(c);
			instructions.push_back(c);
		}
	}

	int floor = 0;
	for (int i = 0; i < instructions.size();) {
		if (instructions[i] == '(') {
			floor = floor + 1;
		}
		else if (instructions[i] == ')') {
			floor = floor - 1;
		}
		i++;
	}
	cout << floor << endl;
}