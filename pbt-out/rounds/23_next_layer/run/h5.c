#define CAT(a,b) a##b
#define SL /##/
int x = 1; SL comment
int y = CAT(1,2);
