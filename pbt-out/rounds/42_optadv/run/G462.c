static int f1(int *p, int *q) { *p += 804; return *q * 2; }
static int f2(int *p, int *q) { *p += 346; return *q * 2; }
static int f3(int *p, int *q) { *p += 700; return *q * 2; }
static int f4(int *p, int *q) { *p += 14; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 479u; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[0], &cells[3]);
    sum += f2(&cells[5], &cells[1]);
    arr[1 % 6] = 83; cells[3] += arr[1 % 6];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    sum += f3(&cells[7], &cells[0]);
    cells[2] = cells[7] + 44;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[5 % 6] = 977; cells[3] += arr[5 % 6];
    arr[6 % 6] = 885; cells[4] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f4(&cells[5], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }