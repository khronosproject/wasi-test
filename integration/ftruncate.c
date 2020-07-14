// { "preopens": { "/scratch": "scratch" } }

#include <assert.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

int main(void) {
	int fd = open("/scratch/file", O_CREAT | O_WRONLY, 0666);
	assert(fd != -1);

	struct stat st;

	assert(fstat(fd, &st) == 0);
	assert(st.st_size == 0);
	assert(lseek(fd, 0, SEEK_CUR) == 0);

	assert(ftruncate(fd, 256) == 0);
	assert(fstat(fd, &st) == 0);
	assert(st.st_size == 256);
	assert(lseek(fd, 0, SEEK_CUR) == 0);

	assert(ftruncate(fd, 512) == 0);
	assert(fstat(fd, &st) == 0);
	assert(st.st_size == 512);
	assert(lseek(fd, 0, SEEK_CUR) == 0);

	assert(ftruncate(fd, 1024) == 0);
	assert(fstat(fd, &st) == 0);
	assert(st.st_size == 1024);
	assert(lseek(fd, 0, SEEK_CUR) == 0);

	assert(ftruncate(fd, 512) == 0);
	assert(fstat(fd, &st) == 0);
	assert(st.st_size == 512);
	assert(lseek(fd, 0, SEEK_CUR) == 0);

	assert(ftruncate(fd, 256) == 0);
	assert(fstat(fd, &st) == 0);
	assert(st.st_size == 256);
	assert(lseek(fd, 0, SEEK_CUR) == 0);

	assert(ftruncate(fd, 0) == 0);
	assert(fstat(fd, &st) == 0);
	assert(st.st_size == 0);
	assert(lseek(fd, 0, SEEK_CUR) == 0);

	assert(close(fd) == 0);

	return 0;
}
