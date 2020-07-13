// { "preopens": { "/scratch": "scratch" } }

#include <assert.h>
#include <stdio.h>

int main(void) {
	char oldname[] = "/scratch/oldfile";
	char newname[] = "/scratch/newfile";

	FILE *oldfile = fopen(oldname, "w");
	assert(oldfile != NULL);
	assert(fclose(oldfile) == 0);

	assert(rename(oldname, newname) == 0);

	FILE *newfile = fopen(newname, "r");
	assert(newfile != NULL);
	assert(fclose(newfile) == 0);

	return 0;
}
