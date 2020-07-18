// { "preopens": { "/scratch": "scratch" } }

#include <assert.h>
#include <errno.h>
#include <sys/stat.h>

int main(void) {
	if (mkdir("/scratch/directory", 0700) != 0) {
		return errno;
	}

	struct stat st;
	assert(stat("/scratch/directory", &st) == 0);
	assert(S_ISDIR(st.st_mode));

	return 0;
}
