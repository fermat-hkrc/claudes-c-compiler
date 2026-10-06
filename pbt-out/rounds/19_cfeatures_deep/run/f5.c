const char *classify(int x) { switch (x) { case 1: return "one"; case 2: case 3: return "few"; default: return "many"; } }
int main(void) { printf("%s %s %s %s\n", classify(1), classify(2), classify(3), classify(9)); return 0; }
