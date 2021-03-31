#include <stdio.h>
#include <string.h>

int main() {
	FILE *filepointer;
	char buffer[7000];
	filepointer = fopen("INPUT", "r");
	fgets(buffer, 7000, (FILE*)filepointer);

	int floor = 0;

	for (int i = 0; i < 6999; i++) {
		if (buffer[i] == '(') {
			floor++;
		}
		else if (buffer[i] == ')') { // the only other thing buffer can be is ) so else if isn't necessary
			floor--;
		}
	}
	printf("%d", floor);
	printf("\n");
	printf("%c", buffer[6999]);
	printf("\n");
}
