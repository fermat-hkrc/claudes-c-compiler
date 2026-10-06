extern int so_global;
extern long so_arr[];
int main(void) { printf("%d %ld\n", so_global, so_arr[1] + so_arr[2]); return 0; }
