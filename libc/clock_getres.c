// {}

#include <assert.h>
#include <time.h>

int main() {
	struct timespec ts;

	assert(clock_getres(CLOCK_REALTIME, &ts) == 0);
	assert(ts.tv_sec + ts.tv_nsec > 0);

	assert(clock_getres(CLOCK_MONOTONIC, &ts) == 0);
	assert(ts.tv_sec + ts.tv_nsec > 0);

	assert(clock_getres(CLOCK_PROCESS_CPUTIME_ID, &ts) == 0);
	assert(ts.tv_sec + ts.tv_nsec > 0);

	assert(clock_getres(CLOCK_THREAD_CPUTIME_ID, &ts) == 0);
	assert(ts.tv_sec + ts.tv_nsec > 0);
}
