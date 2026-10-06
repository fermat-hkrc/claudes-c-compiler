#define CAT(a,b) a##b
#define SL /##/
int x = 1; SL this is a comment
int y = CAT(1,2);
int main(void) { return x + y - 13; }
