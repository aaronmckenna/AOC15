#include <stdio.h>
#include <string.h>

int main() {
	FILE *filepointer;
	char buffer[7000];
	filepointer = fopen("INPUT", "r");
	fgets(buffer, 7000, (FILE*)filepointer);

	int floor = 0;

	for (int i = 0; i < 7000; i++) {
		if (buffer[i] == '(') {
			floor = floor + 1;
		}
		else if (buffer[i] == ')') { // the only other thing buffer can be is ) so else if isn't necessary
			floor = floor - 1;
		}

		if (floor == -1) {
			printf("%d", i);
		}
	}
}
