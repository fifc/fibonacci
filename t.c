#include <stdio.h>

int fibonacci(int i)
{
	if (i < 2)
		return i;
	return fibonacci(i - 2) + fibonacci(i - 1);
}

int main(int argc, char *argv[])
{
	printf("%d\n", fibonacci(45));
}
