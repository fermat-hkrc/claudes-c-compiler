static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[0] + 28;
    static int f1(int *p, int *q) { *p += 763; return *q * 2; }
    sum += f1(&cells[2], &cells[0]);
    static int f2(int *p, int *q) { *p += 583; return *q * 2; }
    sum += f2(&cells[7], &cells[1]);
    arr[2 % 6] = 155; cells[0] += arr[2 % 6];
    static int f3(int *p, int *q) { *p += 38; return *q * 2; }
    sum += f3(&cells[2], &cells[4]);
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    cells[6] = cells[7] + 580;
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 50u; }
    arr[5 % 6] = 954; cells[3] += arr[5 % 6];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }