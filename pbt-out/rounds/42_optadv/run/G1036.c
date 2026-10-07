static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[1] + 936;
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    arr[5 % 6] = 777; cells[2] += arr[5 % 6];
    static int f1(int *p, int *q) { *p += 708; return *q * 2; }
    sum += f1(&cells[0], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f2(int *p, int *q) { *p += 709; return *q * 2; }
    sum += f2(&cells[0], &cells[0]);
    static int f3(int *p, int *q) { *p += 843; return *q * 2; }
    sum += f3(&cells[0], &cells[0]);
    static int f4(int *p, int *q) { *p += 803; return *q * 2; }
    sum += f4(&cells[2], &cells[5]);
    static int f5(int *p, int *q) { *p += 887; return *q * 2; }
    sum += f5(&cells[4], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[7 % 6] = 109; cells[7] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 659u; }
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }