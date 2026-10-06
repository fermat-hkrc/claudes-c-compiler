union U { long l; int i[2]; char c[8]; };
int main(void) { union U u; u.l = 0x0102030405060708L; printf("%d %d %d\n", u.i[0] != 0, u.c[7], (int)sizeof(u)); return 0; }
